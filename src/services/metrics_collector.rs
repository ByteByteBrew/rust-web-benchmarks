use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::{Pid, System};
use tokio::time;

use crate::{traits::MetricsCollector, types::ResourceStats};

pub struct SystemMetricsCollector {
    system: System,
}

impl SystemMetricsCollector {
    pub fn new() -> Self {
        Self {
            system: System::new(),
        }
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
