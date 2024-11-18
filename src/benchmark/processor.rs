use std::{collections::HashMap, io::Write};

use anyhow::Result;

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

    fn generate_table_header(&self, endpoints: &[String]) -> String {
        let mut header = "| Rank ".to_string();
        for endpoint in endpoints {
            header.push_str(&format!("| {} ", endpoint));
        }
        header.push('|');

        let mut separator = "|------|".to_string();
        for _ in endpoints {
            separator.push_str("-----------------|");
        }

        format!("{}\n{}", header, separator)
    }

    fn format_row(
        &self,
        rank: usize,
        endpoint_results: &HashMap<String, Vec<(String, f64)>>,
        endpoints: &[String],
    ) -> String {
        let mut row = format!("| {:4} ", rank);

        for endpoint in endpoints {
            if let Some(results) = endpoint_results.get(endpoint) {
                if let Some((framework, rps)) = results.get(rank - 1) {
                    row.push_str(&format!("| {:9} {:9.0} ", framework, rps));
                } else {
                    row.push_str("|                  ");
                }
            }
        }

        row.push('|');
        row
    }

    fn process_results(
        &self,
        results: &[BenchmarkResult],
    ) -> (Vec<String>, HashMap<usize, HashMap<String, Vec<(String, f64)>>>) {
        let mut concurrency_groups = HashMap::new();
        let mut endpoints = Vec::new();

        for result in results {
            if !endpoints.contains(&result.endpoint) {
                endpoints.push(result.endpoint.clone());
            }
        }
        endpoints.sort();

        for result in results {
            let endpoint_results = concurrency_groups
                .entry(result.concurrency)
                .or_insert_with(HashMap::new)
                .entry(result.endpoint.clone())
                .or_insert_with(Vec::new);

            endpoint_results.push((result.framework.clone(), result.metrics.req_per_sec));
        }

        for endpoint_groups in concurrency_groups.values_mut() {
            for results in endpoint_groups.values_mut() {
                results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            }
        }

        (endpoints, concurrency_groups)
    }
}

impl ResultsProcessor for DefaultResultsProcessor {
    fn print_console_report(&self, results: &[BenchmarkResult]) {
        let (endpoints, concurrency_groups) = self.process_results(results);
        let mut concurrencies: Vec<_> = concurrency_groups.keys().collect();
        concurrencies.sort();

        println!("\n🚀 Performance Rankings (Requests/sec)");

        for &concurrency in concurrencies {
            println!("\n## Concurrency: {}\n", concurrency);
            println!("{}", self.generate_table_header(&endpoints));

            if let Some(endpoint_results) = concurrency_groups.get(&concurrency) {
                let max_rank = endpoint_results.values().map(|v| v.len()).max().unwrap_or(0);

                for rank in 1..=max_rank {
                    println!("{}", self.format_row(rank, endpoint_results, &endpoints));
                }
            }
        }
    }

    fn generate_markdown_report(&self, results: &[BenchmarkResult]) -> Result<()> {
        let mut file = std::fs::File::create("benchmark_results.md")?;

        writeln!(file, "# Web Framework Benchmark Results\n")?;
        writeln!(file, "## Test Configuration\n")?;
        writeln!(file, "- Date: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file, "## Hardware Information\n{}\n", self.metrics_collector.hardware_info())?;

        writeln!(file, "# Performance Rankings (Requests/sec)")?;

        let (endpoints, concurrency_groups) = self.process_results(results);
        let mut concurrencies: Vec<_> = concurrency_groups.keys().collect();
        concurrencies.sort();

        for &concurrency in concurrencies {
            writeln!(file, "\n## Concurrency: {}\n", concurrency)?;
            writeln!(file, "{}", self.generate_table_header(&endpoints))?;

            if let Some(endpoint_results) = concurrency_groups.get(&concurrency) {
                let max_rank = endpoint_results.values().map(|v| v.len()).max().unwrap_or(0);

                for rank in 1..=max_rank {
                    writeln!(file, "{}", self.format_row(rank, endpoint_results, &endpoints))?;
                }
            }
        }

        Ok(())
    }
}
