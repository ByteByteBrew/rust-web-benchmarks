use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::Pid;
use tokio::process::Command;

use crate::{
    services::{ServiceManager, SystemMetricsCollector},
    traits::{BenchmarkExecutor, MetricsCollector, ServiceControl},
    types::*,
};

pub struct DefaultBenchmarkExecutor {
    config: BenchmarkConfig,
}

impl DefaultBenchmarkExecutor {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    async fn execute_wrk(
        &self,
        endpoint: &str,
        concurrency: usize,
        port: u16,
    ) -> Result<WrkOutput> {
        let url = format!("http://localhost:{}{}", port, endpoint);
        let output = Command::new("wrk")
            .args([
                "-t",
                &self.config.threads.to_string(),
                "-c",
                &concurrency.to_string(),
                "-d",
                &format!("{}s", self.config.duration),
                &url,
            ])
            .output()
            .await?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        WrkOutput::parse(&output_str).await
    }
}

#[async_trait]
impl BenchmarkExecutor for DefaultBenchmarkExecutor {
    async fn run_single_test(
        &mut self,
        endpoint: &str,
        concurrency: usize,
        port: u16,
        pid: Pid,
    ) -> Result<(WrkOutput, ResourceStats)> {
        let duration = Duration::from_secs(self.config.duration);

        let metrics_future = {
            let pid = pid;
            let duration = duration;
            let mut collector = SystemMetricsCollector::new();
            tokio::spawn(async move { collector.collect_metrics(pid, duration).await })
        };

        let wrk_future = self.execute_wrk(endpoint, concurrency, port);

        let (wrk_output, metrics_result) = tokio::join!(wrk_future, metrics_future);

        Ok((wrk_output?, metrics_result??))
    }

    async fn run_framework_benchmark(
        &mut self,
        framework: &Framework,
        endpoints: &[&str],
    ) -> Result<Vec<BenchmarkResult>> {
        let mut results = Vec::new();
        let service_manager = ServiceManager::new(&framework.name, framework.port);
        let rounds = self.config.rounds;
        let concurrencies = self.config.concurrencies.clone();

        for &endpoint in endpoints {
            for &concurrency in &concurrencies {
                for round in 1..=rounds {
                    println!(
                        "  Round {}/{}, Concurrency: {}, Endpoint: {}",
                        round, rounds, concurrency, endpoint
                    );

                    let (mut child, pid) = service_manager.start().await?;
                    let (metrics, resources) =
                        self.run_single_test(endpoint, concurrency, framework.port, pid).await?;

                    <ServiceManager as ServiceControl>::stop(&mut child).await?;

                    results.push(BenchmarkResult {
                        framework: framework.name.clone(),
                        endpoint: endpoint.to_string(),
                        round,
                        concurrency,
                        metrics,
                        resources,
                    });
                }
            }
        }

        Ok(results)
    }
}
