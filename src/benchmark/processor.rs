use std::{collections::HashMap, io::Write};

use anyhow::Result;
use colored::*;

use crate::{services::SystemMetricsCollector, traits::ResultsProcessor, types::BenchmarkResult};

pub struct DefaultResultsProcessor {
    metrics_collector: SystemMetricsCollector,
}

impl DefaultResultsProcessor {
    pub fn new() -> Self {
        Self {
            metrics_collector: SystemMetricsCollector::new(),
        }
    }

    fn format_resources(&self, stats: &crate::types::ResourceStats) -> String {
        format!(
            "CPU: {:.1}%-{:.1}% (avg: {:.1}%), MEM: {:.1}-{:.1}MB (avg: {:.1}MB)",
            stats.cpu_min,
            stats.cpu_max,
            stats.cpu_avg,
            stats.mem_min,
            stats.mem_max,
            stats.mem_avg,
        )
    }
}

impl ResultsProcessor for DefaultResultsProcessor {
    fn group_results<'a>(
        &self,
        results: &'a [BenchmarkResult],
    ) -> Vec<((usize, String), Vec<&'a BenchmarkResult>)> {
        let mut endpoint_groups: HashMap<String, Vec<(usize, Vec<&'a BenchmarkResult>)>> =
            HashMap::new();

        for result in results {
            endpoint_groups
                .entry(result.endpoint.clone())
                .or_default()
                .push((result.concurrency, vec![result]));
        }

        let mut final_results = Vec::new();
        let mut endpoints: Vec<String> = endpoint_groups.keys().cloned().collect();
        endpoints.sort();

        for endpoint in endpoints {
            if let Some(mut concurrency_groups) = endpoint_groups.remove(&endpoint) {
                concurrency_groups.sort_by_key(|(concurrency, _)| *concurrency);

                for (concurrency, results) in concurrency_groups {
                    final_results.push(((concurrency, endpoint.clone()), results));
                }
            }
        }

        final_results
    }

    fn print_console_report(&self, results: &[BenchmarkResult]) {
        let grouped = self.group_results(results);

        for ((concurrency, endpoint), results) in grouped.iter() {
            println!("\n📊 Results for {} concurrent users at {}", concurrency, endpoint);
            println!("==============================================");

            for (i, result) in results.iter().enumerate() {
                println!(
                    "\n{}. {} (Round {})",
                    i + 1,
                    result.framework.bright_blue(),
                    result.round
                );
                println!("   Requests/sec: {:.2}", result.metrics.req_per_sec);
                println!(
                    "   Latency: {:.2}ms (avg) / {:.2}ms (max)",
                    result.metrics.latency_avg, result.metrics.latency_max
                );
                println!("   Total Requests: {}", result.metrics.total_requests);
                println!("   {}", self.format_resources(&result.resources));
            }
        }
    }

    fn generate_markdown_report(&self, results: &[BenchmarkResult]) -> Result<()> {
        let mut file = std::fs::File::create("benchmark_results.md")?;
        let grouped = self.group_results(results);

        writeln!(file, "# Web Framework Benchmark Results\n")?;
        writeln!(file, "## Test Configuration\n")?;
        writeln!(file, "- Date: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file, "## Hardware Information\n{}\n", self.metrics_collector.hardware_info())?;

        for ((concurrency, endpoint), results) in grouped.iter() {
            writeln!(file, "## Results for {} concurrent users at {}\n", concurrency, endpoint)?;
            writeln!(
                file,
                "| Framework | Requests/sec | Latency (avg) | Latency (max) | Total Requests | CPU (avg) | Memory (avg) |"
            )?;
            writeln!(
                file,
                "|-----------|--------------|---------------|---------------|----------------|-----------|--------------|"
            )?;

            for result in results {
                writeln!(
                    file,
                    "| {} | {:.2} | {:.2}ms | {:.2}ms | {} | {:.1}% | {:.1}MB |",
                    result.framework,
                    result.metrics.req_per_sec,
                    result.metrics.latency_avg,
                    result.metrics.latency_max,
                    result.metrics.total_requests,
                    result.resources.cpu_avg,
                    result.resources.mem_avg
                )?;
            }
            writeln!(file)?;
        }

        Ok(())
    }
}
