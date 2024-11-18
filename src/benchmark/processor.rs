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
        let mut header = "| Rank | Framework ".to_string();
        for endpoint in endpoints {
            header.push_str(&format!("| {} ", endpoint));
        }
        header.push_str("| Concurrency |");

        let mut separator = "|------|-----------|".to_string();
        for _ in endpoints {
            separator.push_str("-----------|");
        }
        separator.push_str("------------|");

        format!("{}\n{}", header, separator)
    }

    fn format_row(
        &self,
        rank: usize,
        framework: &str,
        endpoint_results: &HashMap<String, f64>,
        endpoints: &[String],
        concurrency: usize,
    ) -> String {
        let mut row = format!("| {:4} | {:9} ", rank, framework);

        for endpoint in endpoints {
            row.push_str(&format!("| {:9.0} ", endpoint_results.get(endpoint).unwrap_or(&0.0)));
        }

        row.push_str(&format!("| {:10} |", concurrency));
        row
    }

    fn process_results(
        &self,
        results: &[BenchmarkResult],
    ) -> (Vec<String>, HashMap<usize, Vec<(String, HashMap<String, f64>)>>) {
        let mut concurrency_groups: HashMap<usize, Vec<(String, HashMap<String, f64>)>> =
            HashMap::new();
        let mut endpoints = Vec::new();

        for result in results {
            if !endpoints.contains(&result.endpoint) {
                endpoints.push(result.endpoint.clone());
            }
        }
        endpoints.sort();

        let mut temp_results: HashMap<(String, usize), HashMap<String, f64>> = HashMap::new();

        for result in results {
            let key = (result.framework.clone(), result.concurrency);
            let framework_results = temp_results.entry(key).or_default();
            framework_results.insert(result.endpoint.clone(), result.metrics.req_per_sec);
        }

        for ((framework, concurrency), endpoint_results) in temp_results {
            if endpoint_results.len() == endpoints.len() {
                let results_vec = concurrency_groups.entry(concurrency).or_default();

                results_vec.push((framework, endpoint_results));
            }
        }

        for results in concurrency_groups.values_mut() {
            results.sort_by(|a, b| {
                let a_total: f64 = a.1.values().sum();
                let b_total: f64 = b.1.values().sum();
                b_total.partial_cmp(&a_total).unwrap()
            });
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

            if let Some(results) = concurrency_groups.get(&concurrency) {
                for (rank, (framework, endpoint_results)) in results.iter().enumerate() {
                    println!(
                        "{}",
                        self.format_row(
                            rank + 1,
                            framework,
                            endpoint_results,
                            &endpoints,
                            concurrency
                        )
                    );
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

        let (endpoints, concurrency_groups) = self.process_results(results);
        let mut concurrencies: Vec<_> = concurrency_groups.keys().collect();
        concurrencies.sort();

        writeln!(file, "# Performance Rankings (Requests/sec)")?;

        for &concurrency in concurrencies {
            writeln!(file, "\n## Concurrency: {}\n", concurrency)?;
            writeln!(file, "{}", self.generate_table_header(&endpoints))?;

            if let Some(results) = concurrency_groups.get(&concurrency) {
                for (rank, (framework, endpoint_results)) in results.iter().enumerate() {
                    writeln!(
                        file,
                        "{}",
                        self.format_row(
                            rank + 1,
                            framework,
                            endpoint_results,
                            &endpoints,
                            concurrency
                        )
                    )?;
                }
            }
        }

        Ok(())
    }
}
