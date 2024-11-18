use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::Pid;
use tokio::process::Child;

use crate::types::*;

#[async_trait]
pub trait ServiceControl {
    async fn start(&self) -> Result<(Child, Pid)>;
    async fn stop(child: &mut Child) -> Result<()>;
}

#[async_trait]
pub trait MetricsCollector {
    async fn collect_metrics(&mut self, pid: Pid, duration: Duration) -> Result<ResourceStats>;
}

#[async_trait]
pub trait BenchmarkExecutor {
    async fn run_single_test(
        &mut self,
        endpoint: &str,
        concurrency: usize,
        port: u16,
        pid: Pid,
    ) -> Result<(WrkOutput, ResourceStats)>;

    async fn run_framework_benchmark(
        &mut self,
        framework: &Framework,
        endpoints: &[&str],
    ) -> Result<Vec<BenchmarkResult>>;
}

pub trait ResultsProcessor {
    fn print_console_report(&self, results: &[BenchmarkResult]);
    fn generate_markdown_report(&self, results: &[BenchmarkResult]) -> Result<()>;
}

#[async_trait]
pub trait BenchmarkOrchestrator {
    async fn discover_frameworks(&self) -> Result<Vec<Framework>>;
    async fn compile_frameworks(&self, frameworks: &[Framework]) -> Result<()>;
    async fn run_benchmarks(&mut self, frameworks: &[Framework]) -> Result<Vec<BenchmarkResult>>;
    fn process_results(&self, results: Vec<BenchmarkResult>) -> Result<()>;
    async fn run(&mut self) -> Result<()>;
}
