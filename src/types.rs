use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct BenchmarkConfig {
    pub(crate) threads: usize,
    pub(crate) duration: u64,
    pub(crate) rounds: usize,
    pub(crate) concurrencies: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Framework {
    pub(crate) name: String,
    pub(crate) port: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkResult {
    pub(crate) framework: String,
    pub(crate) endpoint: String,
    pub(crate) round: usize,
    pub(crate) concurrency: usize,
    pub(crate) metrics: WrkOutput,
    pub(crate) resources: ResourceStats,
}

#[derive(Debug, Clone, Serialize)]
pub struct WrkOutput {
    pub(crate) latency_avg: f64,
    pub(crate) latency_max: f64,
    pub(crate) req_per_sec: f64,
    pub(crate) total_requests: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResourceStats {
    pub(crate) cpu_min: f64,
    pub(crate) cpu_max: f64,
    pub(crate) cpu_avg: f64,
    pub(crate) mem_min: f64,
    pub(crate) mem_max: f64,
    pub(crate) mem_avg: f64,
}

impl BenchmarkConfig {
    pub fn from_env() -> Self {
        Self {
            threads: std::env::var("BENCH_THREADS").ok().and_then(|v| v.parse().ok()).unwrap_or(8),
            duration: std::env::var("BENCH_DURATION")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15),
            rounds: std::env::var("BENCH_ROUNDS").ok().and_then(|v| v.parse().ok()).unwrap_or(1),
            concurrencies: std::env::var("BENCH_CONCURRENCIES")
                .ok()
                .and_then(|v| {
                    v.split(',')
                        .map(|s| s.trim().parse::<usize>())
                        .collect::<std::result::Result<Vec<_>, _>>()
                        .ok()
                })
                .unwrap_or_else(|| vec![64, 256, 512]),
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "Configuration:\n\
             - Threads: {}\n\
             - Duration: {}s\n\
             - Rounds: {}\n\
             - Concurrencies: {:?}",
            self.threads, self.duration, self.rounds, self.concurrencies
        )
    }
}

impl WrkOutput {
    pub async fn parse(output: &str) -> Result<Self> {
        let mut latency_avg = 0.0;
        let mut latency_max = 0.0;
        let mut req_per_sec = 0.0;
        let mut total_requests = 0;

        for line in output.lines() {
            if line.contains("Latency") && !line.contains("Distribution") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    latency_avg = crate::utils::parse_time_to_ms(parts[1])?;
                    latency_max = crate::utils::parse_time_to_ms(parts[3])?;
                }
            } else if line.contains("Requests/sec:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let req_str = parts[1].replace(',', "");
                    req_per_sec = req_str
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!("Failed to parse req/sec: {}", e))?;
                }
            } else if line.contains("requests in") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 1 {
                    let req_str = parts[0].replace(',', "");
                    total_requests = req_str
                        .parse::<u64>()
                        .map_err(|e| anyhow::anyhow!("Failed to parse total requests: {}", e))?;
                }
            }
        }

        Ok(Self {
            latency_avg,
            latency_max,
            req_per_sec,
            total_requests,
        })
    }
}

impl ResourceStats {
    pub fn new() -> Self {
        Self {
            cpu_min: f64::MAX,
            cpu_max: f64::MIN,
            cpu_avg: 0.0,
            mem_min: f64::MAX,
            mem_max: f64::MIN,
            mem_avg: 0.0,
        }
    }

    pub fn update(&mut self, cpu: f64, mem: f64) {
        self.cpu_min = self.cpu_min.min(cpu);
        self.cpu_max = self.cpu_max.max(cpu);
        self.cpu_avg += cpu;
        self.mem_min = self.mem_min.min(mem);
        self.mem_max = self.mem_max.max(mem);
        self.mem_avg += mem;
    }

    pub fn finalize(mut self, samples: usize) -> Self {
        if samples > 0 {
            self.cpu_avg /= samples as f64;
            self.mem_avg /= samples as f64;
        }
        self
    }
}
