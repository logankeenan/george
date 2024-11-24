//! George is an API leveraging AI to make it easy to control a computer with natural language.
//!
//! George runs in an isolated Docker container and uses AI vision to interpret the screen
//! like a human would, executing basic computer commands (mouse, keyboard) to interact with elements.
//! This makes it more resilient to UI changes and able to automate interfaces that traditional tools can't handle.
//!
//! # Example
//!
//! ```rust,no_run
//! use george_ai::George;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut george = George::new("https://your-molmo-llm.com");
//!     george.start().await?;
//!     george.open_chrome("https://some-website.com").await?;
//!     george.click("sign in link").await?;
//!     george.fill_in("input Email text field", "your@email.com").await?;
//!     george.fill_in("input Password text field", "super-secret").await?;
//!     george.click("sign in button").await?;
//!     george.close_chrome().await?;
//!     george.stop().await?;
//!
//!     Ok(())
//! }
//! ```
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
    /// Creates a new instance of George with the specified vision LLM URL.
    ///
    /// # Arguments
    ///
    /// * `vision_llm_url` - The URL of the Molmo vision LLM service.
    pub fn new(vision_llm_url: &str) -> Self {
        let id = Uuid::new_v4();
        let daemon_settings = DaemonSettings::new(vision_llm_url);

        Self {
            id,
            daemon: Daemon::with_settings(daemon_settings),
            virtual_machine: VirtualMachine::new(),
        }
    }

    /// Sets the authentication token for the vision LLM.
    ///
    /// # Arguments
    ///
    /// * `token` - The authentication token for the vision LLM service.
    pub fn set_vision_llm_auth_token(&mut self, token: &str)  {
        self.daemon.settings = self.daemon.settings.clone().set_vision_llm_auth_token(token.to_string());
    }

    /// Creates a new instance of George with custom daemon settings.
    ///
    /// # Arguments
    ///
    /// * `daemon_settings` - Custom settings for the daemon.
    pub fn with_daemon_settings(daemon_settings: DaemonSettings) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            daemon: Daemon::with_settings(daemon_settings),
            virtual_machine: VirtualMachine::new(),
        }
    }

    /// Starts George by initializing the virtual machine and daemon.
    ///
    /// This method must be called before performing any automation tasks.  It will
    /// spin up a Docker container for you to interact with.
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.virtual_machine.start().await?;

        if let Some(port) = self.virtual_machine.port.as_ref() {
            self.daemon.set_port(port.clone());

            println!("Daemon running at http://localhost:{}", port);
            Ok(self.daemon.ready().await?)
        } else {
            Err("Failed to get port from virtual machine".into())
        }
    }

    /// Stops George by shutting down the docker container.
    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.virtual_machine.stop().await
    }

    /// Fills in a form field identified by the given selector with the provided text.
    ///
    /// # Arguments
    ///
    /// * `selector` - A natural language description of the form field (e.g., "input Email text field").
    /// * `with` - The text to enter into the field.
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

    /// Takes a screenshot of the current state of the docker container.
    pub async fn screenshot(&self) -> Result<Bytes, DaemonError> {
        self.daemon.screenshot().await
    }

    /// Clicks on an element identified by the given selector.
    ///
    /// # Arguments
    ///
    /// * `selector` - A natural language description of the element to click (e.g., "sign in button").
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

    /// Waits until the specified text is visible on the screen.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to wait for.
    pub async fn wait_until_text_is_visible(&self, text: &str) -> Result<(), DaemonError> {
        let timeout = Duration::from_secs(5);
        let start = Instant::now();

        while start.elapsed() < timeout {
            match self.daemon.is_text_visible(text).await {
                Ok(result) => {
                    match result {
                        true => return Ok(()),
                        false => {
                            println!("Failed determine if text is visible '{}'. Retrying...", text);
                            sleep(Duration::from_millis(10)).await;
                            continue;
                        }
                    }
                },
                Err(_e) => {
                    println!("Failed determine if text is visible '{}'. Retrying...", text);
                    sleep(Duration::from_millis(10)).await;
                    continue;
                }
            }
        }

        Err(DaemonError::Unexpected(String::from("Text is not visible")))
    }

    /// Executes a command in the virtual machine.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute.
    /// * `wait_for_output` - Whether to wait for the command output.
    pub async fn execute(&self, command: &str, wait_for_output: bool) -> Result<String, VirtualMachineError> {
        self.virtual_machine.execute(command, wait_for_output).await
    }

    pub async fn coordinate_of_from_prompt(&self, prompt: &str) -> Result<(u32, u32), DaemonError> {
        self.daemon.coordinate_of_from_prompt(prompt).await
    }

    /// Opens Chrome in the virtual machine and navigates to the specified URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to open in Chrome.
    pub async fn open_chrome(&self, url: &str) -> Result<(), VirtualMachineError> {
        self.execute(

            format!("google-chrome {} --no-sandbox --no-first-run --no-default-browser-check", url).as_str(),
            false,
        ).await?;

        Ok(())
    }

    /// Closes Chrome in the virtual machine.
    pub async fn close_chrome(&self) -> Result<(), VirtualMachineError> {
        self.execute("pkill google-chrome", true).await?;

        Ok(())
    }
}
