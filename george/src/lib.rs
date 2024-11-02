mod daemon;
mod virtual_machine;

use crate::daemon::{Daemon, DaemonError, DaemonSettings};
use crate::virtual_machine::{VirtualMachine, VirtualMachineError};
use bytes::Bytes;
use std::error::Error;
use std::time::Duration;
use tokio::time::{sleep, Instant};
use uuid::Uuid;

pub struct George {
    pub daemon: Daemon,
    pub id: Uuid,
    virtual_machine: VirtualMachine,
}

impl George {
    pub fn new(vision_llm_url: &str) -> Self {
        let id = Uuid::new_v4();
        let daemon_settings = DaemonSettings::new(vision_llm_url);

        Self {
            id,
            daemon: Daemon::with_settings(daemon_settings),
            virtual_machine: VirtualMachine::new(),
        }
    }

    pub fn with_daemon_settings(daemon_settings: DaemonSettings) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            daemon: Daemon::with_settings(daemon_settings),
            virtual_machine: VirtualMachine::new(),
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.virtual_machine.start().await?;

        if let Some(port) = self.virtual_machine.port.as_ref() {
            self.daemon.set_port(port.clone());
            Ok(self.daemon.ready().await?)
        } else {
            Err("Failed to get port from virtual machine".into())
        }
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.virtual_machine.stop().await
    }

    pub async fn fill_in(&self, selector: &str, with: &str) -> Result<(), DaemonError> {
        let timeout = Duration::from_secs(10);
        let start = Instant::now();

        while start.elapsed() < timeout {
            match self.daemon.click(selector).await {
                Ok(_) => return self.daemon.type_text(with).await,
                Err(e) => {
                    match e {
                        DaemonError::FailedToParseCoordinates(_) => {
                            println!("Failed to parse coordinates for selector '{}'. Retrying...", selector);
                            sleep(Duration::from_millis(10)).await;
                            continue;
                        }
                        _ => return Err(e),
                    }
                }
            }
        }

        Err(DaemonError::SelectorTimeout(String::from(selector)))
    }

    pub async fn screenshot(&self) -> Result<Bytes, DaemonError> {
        self.daemon.screenshot().await
    }

    pub async fn click(&self, selector: &str) -> Result<(), DaemonError> {
        let timeout = Duration::from_secs(10);
        let start = Instant::now();

        while start.elapsed() < timeout {
            match self.daemon.click(selector).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    match e {
                        DaemonError::FailedToParseCoordinates(_) => {
                            println!("Failed to parse coordinates for selector '{}'. Retrying...", selector);
                            sleep(Duration::from_millis(10)).await;
                            continue;
                        }
                        _ => return Err(e),
                    }
                }
            }
        }

        Err(DaemonError::SelectorTimeout(String::from(selector)))
    }

    pub async fn is_visible(&self, selector: &str) -> Result<bool, DaemonError> {
        let timeout = Duration::from_secs(10);
        let start = Instant::now();

        while start.elapsed() < timeout {
            match self.daemon.coordinate_of(selector).await {
                Ok(_) => return Ok(true),
                Err(e) => {
                    match e {
                        DaemonError::FailedToParseCoordinates(_) => {
                            println!("Failed to parse coordinates for selector '{}'. Retrying...", selector);
                            sleep(Duration::from_millis(10)).await;
                            continue;
                        }
                        _ => return Err(e),
                    }
                }
            }
        }

        Ok(false)
    }

    pub async fn execute(&self, command: &str, wait_for_output: bool) -> Result<String, VirtualMachineError> {
        self.virtual_machine.execute(command, wait_for_output).await
    }

    pub async fn coordinate_of_from_prompt(&self, prompt: &str) -> Result<(u32, u32), DaemonError> {
        self.daemon.coordinate_of_from_prompts(prompt).await
    }

    pub async fn open_firefox(&self, url: &str) -> Result<(), VirtualMachineError> {
        self.execute(
            format!("firefox {} --width=1024 --height=768 --display=:99", url).as_str(),
            false,
        ).await?;

        Ok(())
    }

    pub async fn close_firefox(&self) -> Result<(), VirtualMachineError> {
        self.execute("pkill firefox", false).await?;

        Ok(())
    }
}
