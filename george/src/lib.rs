mod daemon;
mod virtual_machine;

use crate::daemon::{Daemon, DaemonError};
use crate::virtual_machine::VirtualMachine;
use bytes::Bytes;
use std::error::Error;
use uuid::Uuid;

pub struct George {
    pub daemon: Daemon,
    pub id: Uuid,
    virtual_machine: VirtualMachine,
}

impl Default for George {
    fn default() -> Self {
        Self::new()
    }
}

impl George {
    pub fn new() -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            daemon: Daemon::new(),
            virtual_machine: VirtualMachine::new(),
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.virtual_machine.start().await?;

        if let Some(port) = self.virtual_machine.port.as_ref() {
            self.daemon.set_port(port.clone());
            Ok(())
        } else {
            Err("Failed to get port from virtual machine".into())
        }
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.virtual_machine.stop().await
    }

    pub async fn fill_in(&self, selector: &str, with: &str) -> Result<(), DaemonError> {
        self.daemon.click(selector).await?;
        self.type_text(with).await
    }

    pub async fn screenshot(&self) -> Result<Bytes, DaemonError> {
        self.daemon.screenshot().await
    }

    pub async fn type_text(&self, text: &str) -> Result<(), DaemonError> {
        self.daemon.type_text(text).await
    }

    pub async fn click(&self, selector: &str) -> Result<(), DaemonError> {
        self.daemon.click(selector).await
    }

    pub async fn coordinate_of(&self, selector: &str) -> Result<(u32, u32), DaemonError> {
        self.daemon.coordinate_of(selector).await
    }
}
