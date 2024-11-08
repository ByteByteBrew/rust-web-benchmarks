//! Web 框架基准测试工具
//!
//! 这个工具用于对比测试不同 Rust web 框架的性能表现。
//! 主要测试指标包括:
//! - 请求吞吐量 (QPS)
//! - 响应延迟 (平均值/分位数)
//! - CPU 使用率
//! - 内存占用

use std::{
    io::Write,
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::Result;
use colored::*;
use serde::Serialize;
use sysinfo::{Pid, System};
use tokio::time;

/// 基准测试配置
///
/// 包含测试所需的所有参数配置，支持从环境变量加载
#[derive(Debug, Clone)]
struct BenchmarkConfig {
    threads: usize,            // 压测线程数
    duration: u64,             // 每轮测试持续时间(秒)
    rounds: usize,             // 测试轮次
    concurrencies: Vec<usize>, // 并发用户数列表
}

impl BenchmarkConfig {
    /// 从环境变量加载配置
    ///
    /// # 环境变量
    /// - BENCH_THREADS: 压测线程数，默认 8
    /// - BENCH_ROUNDS: 测试轮次，默认 1
    fn from_env() -> Self {
        Self {
            threads: std::env::var("BENCH_THREADS").ok().and_then(|v| v.parse().ok()).unwrap_or(8),
            duration: 15,
            rounds: std::env::var("BENCH_ROUNDS").ok().and_then(|v| v.parse().ok()).unwrap_or(1),
            concurrencies: vec![64, 256, 512],
        }
    }
}

/// 系统资源使用统计
///
/// 记录进程的 CPU 和内存使用情况，包括最小值、最大值和平均值
#[derive(Debug, Clone)]
struct ResourceStats {
    cpu_min: f64,
    cpu_max: f64,
    cpu_avg: f64,
    mem_min: f64,
    mem_max: f64,
    mem_avg: f64,
}

impl ResourceStats {
    const fn new() -> Self {
        Self {
            cpu_min: f64::MAX,
            cpu_max: f64::MIN,
            cpu_avg: 0.0,
            mem_min: f64::MAX,
            mem_max: f64::MIN,
            mem_avg: 0.0,
        }
    }

    /// 更新统计数据
    ///
    /// # Arguments
    /// * `cpu` - CPU 使用率百分比
    /// * `mem` - 内存使用量(MB)
    fn update(&mut self, cpu: f64, mem: f64) {
        self.cpu_min = self.cpu_min.min(cpu);
        self.cpu_max = self.cpu_max.max(cpu);
        self.cpu_avg += cpu;
        self.mem_min = self.mem_min.min(mem);
        self.mem_max = self.mem_max.max(mem);
        self.mem_avg += mem;
    }

    /// 完成统计，计算平均值
    ///
    /// # Arguments
    /// * `samples` - 样本数量
    fn finalize(mut self, samples: usize) -> Self {
        if samples > 0 {
            self.cpu_avg /= samples as f64;
            self.mem_avg /= samples as f64;
        } else {
            self.cpu_min = 0.0;
            self.cpu_max = 0.0;
            self.cpu_avg = 0.0;
            self.mem_min = 0.0;
            self.mem_max = 0.0;
            self.mem_avg = 0.0;
        }
        self
    }
}

/// wrk 输出结果解析
#[derive(Debug)]
struct WrkOutput {
    latency_avg: f64,
    latency_stdev: f64,
    latency_max: f64,
    latency_p50: f64,
    latency_p90: f64,
    latency_p99: f64,
    req_per_sec: f64,
    total_requests: u64,
    total_bytes: u64,
    errors: u64,
}

impl WrkOutput {
    /// 解析 wrk 输出文本
    ///
    /// # Arguments
    /// * `output` - wrk 命令输出的文本
    ///
    /// # Returns
    /// 解析后的结果结构体
    fn parse(output: &str) -> Result<Self> {
        let mut result = Self {
            latency_avg: 0.0,
            latency_stdev: 0.0,
            latency_max: 0.0,
            latency_p50: 0.0,
            latency_p90: 0.0,
            latency_p99: 0.0,
            req_per_sec: 0.0,
            total_requests: 0,
            total_bytes: 0,
            errors: 0,
        };

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // 解析延迟统计
            if line.starts_with("Latency") && !line.contains("Distribution") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    result.latency_avg = parse_time_to_ms(parts[1])?;
                    result.latency_stdev = parse_time_to_ms(parts[2])?;
                    result.latency_max = parse_time_to_ms(parts[3])?;
                }
            }
            // 解析延迟分位数
            else if line.contains("Latency Distribution") {
                continue;
            } else if line.starts_with("50%") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(&value) = parts.get(1) {
                    result.latency_p50 = parse_time_to_ms(value)?;
                }
            } else if line.starts_with("90%") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(&value) = parts.get(1) {
                    result.latency_p90 = parse_time_to_ms(value)?;
                }
            } else if line.starts_with("99%") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(&value) = parts.get(1) {
                    result.latency_p99 = parse_time_to_ms(value)?;
                }
            }
            // 解析请求统计
            else if line.contains("Requests/sec") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(value) = parts.get(1) {
                    result.req_per_sec = value.parse::<f64>()?;
                }
            }
            // 解析总请求数和传输字节数
            else if line.contains("requests in") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 1 {
                    if let Ok(requests) = parts[0].parse::<u64>() {
                        result.total_requests = requests;
                    }
                }
                if parts.len() >= 4 {
                    if let Ok(bytes) = parts[3].parse::<u64>() {
                        result.total_bytes = bytes;
                    }
                }
            }
            // 解析错误数
            else if line.contains("Non-2xx") || line.contains("Socket errors") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(value) = parts.get(2) {
                    if let Ok(count) = value.parse::<u64>() {
                        result.errors += count;
                    }
                }
            }
        }

        Ok(result)
    }
}

/// 解析时间字符串为毫秒数
fn parse_time_to_ms(time_str: &str) -> Result<f64> {
    // 处理空字符串
    if time_str.is_empty() {
        return Ok(0.0);
    }

    // 查找数字部分结束的位置
    let num_end = time_str
        .find(|c: char| !c.is_digit(10) && c != '.' && c != '-')
        .unwrap_or(time_str.len());

    // 解析数字部分
    let value: f64 = time_str[..num_end].parse().map_err(|e| {
        anyhow::anyhow!("Failed to parse time value '{}': {}", &time_str[..num_end], e)
    })?;

    // 解析单位部分
    let unit = time_str[num_end..].trim();
    match unit {
        "us" => Ok(value / 1000.0),
        "ms" => Ok(value),
        "s" => Ok(value * 1000.0),
        "" => Ok(value), // 没有单位时假设为毫秒
        unit => Err(anyhow::anyhow!("Unknown time unit: {}", unit)),
    }
}

/// 获取工作空间成员
fn get_workspace_members() -> Result<Vec<String>> {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")?;
    let value = cargo_toml.parse::<toml::Table>()?;

    Ok(value["workspace"]["members"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("workspace.members not found"))?
        .iter()
        .filter_map(|v| v.as_str())
        .filter(|&name| name != "common")
        .map(String::from)
        .collect())
}

/// 编译指定项目
async fn build_project(member: &str) -> Result<()> {
    let manifest_path = format!("./{}/Cargo.toml", member);
    let status = Command::new("cargo")
        .args(["build", "--release", "--manifest-path", &manifest_path])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to build {}", member);
    }
    Ok(())
}

/// 启动服务进程
async fn start_service(name: &str) -> Result<(std::process::Child, Pid)> {
    let child = Command::new(format!("./target/release/{}", name))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let pid = Pid::from_u32(child.id());
    time::sleep(Duration::from_secs(2)).await;
    Ok((child, pid))
}

/// 单次测试结果
#[derive(Debug, Serialize, Clone)]
struct BenchmarkResult {
    framework: String,
    endpoint: String,
    round: usize,
    concurrency: usize,
    requests_per_sec: f64,
    latency_avg: f64,
    latency_p90: f64,
    latency_max: f64,
    total_requests: u64,
    errors: u64,
    cpu_min: f64,
    cpu_max: f64,
    cpu_avg: f64,
    mem_min: f64,
    mem_max: f64,
    mem_avg: f64,
}

/// 执行单次 wrk 测试
///
/// # Arguments
/// * `endpoint` - 测试的 API 端点
/// * `config` - 基准测试配置
/// * `concurrency` - 并发用户数
/// * `sys` - 系统信息收集器
/// * `pid` - 被测试进程 ID
///
/// # Returns
/// 测试结果和资源使用统计
async fn run_wrk_test(
    endpoint: &str,
    config: &BenchmarkConfig,
    concurrency: usize,
    sys: &mut System,
    pid: Pid,
) -> Result<(WrkOutput, ResourceStats)> {
    let mut stats = ResourceStats::new();
    let mut samples = 0;

    // 监控资源使用
    let monitor_future = async {
        let start = tokio::time::Instant::now();
        while start.elapsed() < Duration::from_secs(config.duration) {
            sys.refresh_all();
            if let Some(process) = sys.process(pid) {
                let cpu = process.cpu_usage() as f64;
                let mem = process.memory() as f64 / 1024.0 / 1024.0;
                stats.update(cpu, mem);
                samples += 1;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        stats.finalize(samples)
    };

    // 执行 wrk 测试
    let wrk_future = async {
        let output = Command::new("wrk")
            .args([
                "-t",
                &config.threads.to_string(),
                "-c",
                &concurrency.to_string(),
                "-d",
                &format!("{}s", config.duration),
                "--timeout",
                "8s",
                &format!("http://127.0.0.1:8080{}", endpoint),
            ])
            .output()?;
        WrkOutput::parse(&String::from_utf8_lossy(&output.stdout))
    };

    let (stats, wrk_result) = tokio::join!(monitor_future, wrk_future);
    Ok((wrk_result?, stats))
}

/// 执行完整的基准测试
///
/// # Arguments
/// * `framework` - 框架名称
/// * `config` - 基准测试配置
/// * `endpoints` - 测试端点列表
///
/// # Returns
/// 所有测试结果的集合
async fn run_benchmark(
    framework: &str,
    config: &BenchmarkConfig,
    endpoints: &[&str],
) -> Result<Vec<BenchmarkResult>> {
    println!("🚀 Starting {} service...", framework.bright_blue());
    let (mut child, pid) = start_service(framework).await?;
    let mut sys = System::new_all();
    let mut results = Vec::new();

    for round in 1..=config.rounds {
        println!("📊 Round {}/{}", round, config.rounds);

        for &concurrency in &config.concurrencies {
            for &endpoint in endpoints {
                print!("Testing {} ({} users)... ", endpoint, concurrency);
                std::io::stdout().flush()?;

                // 执行单次测试
                let (wrk_result, stats) =
                    run_wrk_test(endpoint, config, concurrency, &mut sys, pid).await?;

                // 收集结果
                results.push(BenchmarkResult {
                    framework: framework.to_string(),
                    endpoint: endpoint.to_string(),
                    round,
                    concurrency,
                    requests_per_sec: wrk_result.req_per_sec,
                    latency_avg: wrk_result.latency_avg,
                    latency_p90: wrk_result.latency_p90,
                    latency_max: wrk_result.latency_max,
                    total_requests: wrk_result.total_requests,
                    errors: wrk_result.errors,
                    cpu_min: stats.cpu_min,
                    cpu_max: stats.cpu_max,
                    cpu_avg: stats.cpu_avg,
                    mem_min: stats.mem_min,
                    mem_max: stats.mem_max,
                    mem_avg: stats.mem_avg,
                });
                println!("✓");

                // 测试间冷却
                time::sleep(Duration::from_secs(10)).await;
            }
        }
    }

    // 清理资源
    println!("💤 Stopping {} service...", framework);
    child.kill()?;
    child.wait()?;
    time::sleep(Duration::from_secs(1)).await;

    Ok(results)
}

/// 格式化并打印测试结果表格
///
/// # Arguments
/// * `results` - 测试结果集合
fn print_results(results: &[BenchmarkResult]) {
    const SEPARATOR: &str = "=";
    const HEADER: &str = "-";
    const WIDTH: usize = 120;

    let separator_line = SEPARATOR.repeat(WIDTH);
    let header_line = HEADER.repeat(WIDTH);

    println!("\n{}", separator_line);
    println!("🎯 Benchmark Results");
    println!("{}", separator_line);

    // 表头
    println!(
        "{:<15} {:<10} {:<8} {:<12} {:<10} {:<10} {:<10} {:<20} {:<20}",
        "Framework",
        "Endpoint",
        "Round",
        "Concurrency",
        "Req/s",
        "Avg(ms)",
        "Total Reqs",
        "CPU%(min/max/avg)",
        "Mem(MB)(min/max/avg)"
    );
    println!("{}", header_line);

    // 结果行
    for result in results {
        println!(
            "{:<15} {:<10} {:<8} {:<12} {:<10.0} {:<10.2} {:<10} {:<6.1}/{:<6.1}/{:<6.1} {:<6.1}/{:<6.1}/{:<6.1}",
            result.framework,
            result.endpoint,
            result.round,
            result.concurrency,
            result.requests_per_sec,
            result.latency_avg,
            result.total_requests,
            result.cpu_min,
            result.cpu_max,
            result.cpu_avg,
            result.mem_min,
            result.mem_max,
            result.mem_avg,
        );
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 加载配置
    let config = BenchmarkConfig::from_env();
    let members = get_workspace_members()?;
    println!("Found {} frameworks to benchmark", members.len());

    // 编译所有项目
    for member in &members {
        build_project(member).await?;
    }

    // 执行测试
    let endpoints = vec!["/plaintext", "/json", "/fortunes"];
    let mut all_results = Vec::new();

    for member in members {
        let results = run_benchmark(&member, &config, &endpoints).await?;
        all_results.extend(results);
    }

    // 输出结果
    print_results(&all_results);
    Ok(())
}
