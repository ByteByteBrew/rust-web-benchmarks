mod benchmark;
mod error;
mod services;
mod traits;
mod types;
mod utils;

use anyhow::Result;

use crate::{
    benchmark::DefaultBenchmarkOrchestrator, traits::BenchmarkOrchestrator, types::BenchmarkConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = BenchmarkConfig::from_env();
    println!("{}", config.describe());

    let mut orchestrator = DefaultBenchmarkOrchestrator::new(config);

    match orchestrator.run().await {
        Ok(_) => {
            println!("\n✨ Benchmark completed successfully!");
            println!("Check benchmark_results.md for detailed results.");
            Ok(())
        }
        Err(e) => {
            eprintln!("\n❌ Benchmark failed: {}", e);
            eprintln!("Try running with RUST_BACKTRACE=1 for more details.");
            Err(e)
        }
    }
}
