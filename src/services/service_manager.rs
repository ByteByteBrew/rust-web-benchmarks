use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::Pid;
use tokio::{process::Command, time};

use crate::traits::ServiceControl;

pub struct ServiceManager {
    binary_path: String,
}

impl ServiceManager {
    pub fn new(framework: &str, _port: u16) -> Self {
        Self {
            binary_path: format!("./target/release/{}", framework),
        }
    }
}

#[async_trait]
impl ServiceControl for ServiceManager {
    async fn start(&self) -> Result<(tokio::process::Child, Pid)> {
        let child = Command::new(&self.binary_path)
            .kill_on_drop(true)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        let pid = Pid::from_u32(child.id().unwrap());
        time::sleep(Duration::from_secs(2)).await;
        Ok((child, pid))
    }

    async fn stop(child: &mut tokio::process::Child) -> Result<()> {
        child.kill().await?;
        child.wait().await?;
        time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }
}
