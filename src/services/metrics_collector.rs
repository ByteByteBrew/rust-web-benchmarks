use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::{Pid, System};
use tokio::time;

use crate::{traits::MetricsCollector, types::ResourceStats};

pub struct SystemMetricsCollector {
    system: System,
    cpu_cores: usize,
    cpu_name: String,
    total_memory: u64,
}

impl SystemMetricsCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let cpu_cores = system.cpus().len();
        let cpu_name = system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let total_memory = system.total_memory() / 1024 / 1024; // Convert to MB
        Self {
            system,
            cpu_cores,
            cpu_name,
            total_memory,
        }
    }

    pub fn hardware_info(&self) -> String {
        format!(
            "- CPU: {} ({} cores)\n- Memory: {} MB",
            self.cpu_name, self.cpu_cores, self.total_memory,
        )
    }

    pub fn cpu_cores(&self) -> usize {
        self.cpu_cores
    }
}

#[async_trait]
impl MetricsCollector for SystemMetricsCollector {
    async fn collect_metrics(&mut self, pid: Pid, duration: Duration) -> Result<ResourceStats> {
        let mut stats = ResourceStats::new();
        let mut samples = 0;
        let start = tokio::time::Instant::now();

        while start.elapsed() < duration {
            self.system.refresh_all();
            if let Some(process) = self.system.process(pid) {
                let cpu = process.cpu_usage() as f64;
                let mem = process.memory() as f64 / 1024.0 / 1024.0;
                stats.update(cpu, mem);
                samples += 1;
            }
            time::sleep(Duration::from_millis(100)).await;
        }

        Ok(stats.finalize(samples))
    }
}
