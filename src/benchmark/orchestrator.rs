use anyhow::Result;
use async_trait::async_trait;
use colored::*;
use tokio::process::Command;

use crate::traits::{BenchmarkExecutor, BenchmarkOrchestrator, ResultsProcessor}; /* 添加这两个 trait */
use crate::{
    benchmark::{DefaultBenchmarkExecutor, DefaultResultsProcessor},
    error::BenchmarkError,
    types::*,
};

pub struct DefaultBenchmarkOrchestrator {
    executor: DefaultBenchmarkExecutor,
    results_processor: DefaultResultsProcessor,
}

impl DefaultBenchmarkOrchestrator {
    /// 默认服务端口
    const DEFAULT_PORT: u16 = 8080;
    /// 测试的端点列表
    const ENDPOINTS: [&'static str; 3] = ["/plaintext", "/json", "/fortunes"];

    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            executor: DefaultBenchmarkExecutor::new(config),
            results_processor: DefaultResultsProcessor::new(),
        }
    }

    async fn get_workspace_members() -> Result<Vec<String>> {
        let cargo_toml = tokio::fs::read_to_string("Cargo.toml").await?;
        let value = cargo_toml.parse::<toml::Table>()?;

        Ok(value["workspace"]["members"]
            .as_array()
            .ok_or_else(|| BenchmarkError::Framework("workspace.members not found".to_string()))?
            .iter()
            .filter_map(|v| v.as_str())
            .filter(|&name| name != "common")
            .map(String::from)
            .collect())
    }

    async fn build_project(member: &str) -> Result<()> {
        println!("🔨 Compiling {}...", member);
        let manifest_path = format!("{}/Cargo.toml", member);

        let status = Command::new("cargo")
            .args(["build", "--release", "--manifest-path", &manifest_path])
            .status()
            .await
            .map_err(|e| BenchmarkError::Execution(format!("Failed to execute cargo: {}", e)))?;

        if !status.success() {
            return Err(BenchmarkError::Framework(format!("Failed to compile {}", member)).into());
        }
        Ok(())
    }
}

#[async_trait]
impl BenchmarkOrchestrator for DefaultBenchmarkOrchestrator {
    async fn discover_frameworks(&self) -> Result<Vec<Framework>> {
        let members = Self::get_workspace_members().await?;
        println!("Found {} frameworks to benchmark", members.len().to_string().bright_green());

        Ok(members
            .into_iter()
            .map(|name| Framework {
                name,
                port: Self::DEFAULT_PORT,
            })
            .collect())
    }

    async fn compile_frameworks(&self, frameworks: &[Framework]) -> Result<()> {
        for framework in frameworks {
            Self::build_project(&framework.name).await?;
        }
        Ok(())
    }

    async fn run_benchmarks(&mut self, frameworks: &[Framework]) -> Result<Vec<BenchmarkResult>> {
        let endpoints = Self::ENDPOINTS;
        let mut all_results = Vec::new();

        for framework in frameworks {
            println!("\n🚀 Benchmarking {}...", framework.name.bright_blue());
            let results = self.executor.run_framework_benchmark(framework, &endpoints).await?;
            all_results.extend(results);
        }

        Ok(all_results)
    }

    fn process_results(&self, results: Vec<BenchmarkResult>) -> Result<()> {
        self.results_processor.print_console_report(&results);
        self.results_processor.generate_markdown_report(&results)?;
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        println!("\n🎯 Starting Web Framework Benchmark");
        println!("==================================");

        let frameworks = self.discover_frameworks().await?;
        if frameworks.is_empty() {
            return Err(anyhow::anyhow!("No frameworks found"));
        }

        self.compile_frameworks(&frameworks).await?;

        let results = self.run_benchmarks(&frameworks).await?;
        if results.is_empty() {
            return Err(anyhow::anyhow!("No benchmark results collected"));
        }

        self.process_results(results)?;

        println!("\n✨ Benchmark completed successfully!");
        Ok(())
    }
}
