use bollard::{
    network::CreateNetworkOptions,
    models::{HostConfig, PortBinding},
    image::BuildImageOptions,
    container::{Config, CreateContainerOptions, StartContainerOptions},
    exec::{CreateExecOptions, StartExecOptions},
    Docker,
};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Duration;
use uuid::Uuid;
use futures_util::StreamExt;
use tar::Builder;
use thiserror::Error;
use tokio::time::sleep;

#[derive(Error, Debug)]
pub enum VirtualMachineError {
    #[error("Docker error: {0}")]
    Docker(#[from] bollard::errors::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to get dynamic port")]
    Port,
    #[error("Build error: {0}")]
    Build(String),
}


pub struct VirtualMachine {
    docker: Docker,
    container_name: String,
    pub port: Option<String>,
    network_name: String,
    id: Uuid,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            docker: Docker::connect_with_local_defaults().expect("Failed to connect to Docker"),
            container_name: format!("george-daemon-container-{}", id),
            port: None,
            network_name: format!("george-network-{}", id),
        }
    }

    pub async fn start(&mut self) -> Result<(), VirtualMachineError> {
        let image_name = format!("george-daemon-image-{}", self.id);

        self.build_image(&image_name).await?;
        self.create_network().await?;
        self.create_container(&image_name).await?;
        self.docker.start_container(&self.container_name, None::<StartContainerOptions<String>>).await?;
        self.extract_port().await?;

        Ok(())
    }

    async fn build_image(&self, image_name: &str) -> Result<(), VirtualMachineError> {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let tar_path = Path::new("/tmp/george-daemon.tar");
        let mut builder = Builder::new(File::create(tar_path)?);
        builder.append_dir_all(".", project_root)?;
        builder.finish()?;

        let tar_contents = std::fs::read(tar_path)?;
        let build_options = BuildImageOptions::<&str> { dockerfile: "Dockerfile", t: image_name, ..Default::default() };

        let mut build_stream = self.docker.build_image(
            build_options,
            None,
            Some(tar_contents.into()),
        );

        while let Some(build_result) = build_stream.next().await {
            match build_result {
                Ok(_) => {},
                Err(e) => return Err(VirtualMachineError::Build(format!("{:?}", e))),
            }
        }

        Ok(())
    }

    async fn create_network(&self) -> Result<(), VirtualMachineError> {
        self.docker.create_network(CreateNetworkOptions {
            name: self.network_name.as_str(),
            ..Default::default()
        }).await?;

        Ok(())
    }

    async fn create_container(&self, image_name: &str) -> Result<(), VirtualMachineError> {
        let mut exposed_ports = HashMap::new();
        exposed_ports.insert(String::from("3000/tcp"), HashMap::new());

        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            String::from("3000/tcp"),
            Some(vec![PortBinding {
                host_ip: Some(String::from("0.0.0.0")),
                host_port: None,
            }]),
        );

        let host_config = HostConfig {
            port_bindings: Some(port_bindings),
            network_mode: Some(self.network_name.clone()),
            ..Default::default()
        };

        self.docker.create_container(
            Some(CreateContainerOptions { name: self.container_name.clone(), platform: None }),
            Config {
                image: Some(image_name.to_string()),
                exposed_ports: Some(exposed_ports),
                host_config: Some(host_config),
                env: Some(vec!["DISPLAY=:99".to_string()]),
                cmd: Some(vec![
                    String::from("sh"), String::from("-c"),
                    String::from("Xvfb :99 -screen 0 1024x768x16 & sleep 2 && ./george-daemon")
                ]),
                ..Default::default()
            },
        ).await?;

        Ok(())
    }

    pub async fn execute(&self, command: &str, wait_for_output: bool) -> Result<String, VirtualMachineError> {
        let container_info = self.docker.inspect_container(&self.container_name, None).await?;
        if !container_info.state.unwrap().running.unwrap_or(false) {
            return Err(VirtualMachineError::Docker(bollard::errors::Error::IOError {
                err: std::io::Error::new(std::io::ErrorKind::Other, "Container is not running"),
            }));
        }

        let exec = self.docker.create_exec(
            &self.container_name,
            CreateExecOptions {
                cmd: Some(vec!["sh", "-c", command]),
                attach_stdout: Some(wait_for_output),
                attach_stderr: Some(wait_for_output),
                ..Default::default()
            },
        ).await?;

        let output = self.docker.start_exec(&exec.id, None::<StartExecOptions>).await?;

        if wait_for_output {
            if let bollard::exec::StartExecResults::Attached { mut output, .. } = output {
                let mut result = String::new();
                while let Some(Ok(output_chunk)) = output.next().await {
                    result.push_str(&output_chunk.to_string());
                }
                Ok(result)
            } else {
                Err(VirtualMachineError::Docker(bollard::errors::Error::IOError {
                    err: std::io::Error::new(std::io::ErrorKind::Other, "Failed to get command output"),
                }))
            }
        } else {
            Ok(String::from("Command started, not waiting for output"))
        }
    }

    async fn extract_port(&mut self) -> Result<(), VirtualMachineError> {
        const MAX_RETRIES: u32 = 10;
        const RETRY_DELAY: Duration = Duration::from_millis(200);

        for attempt in 1..MAX_RETRIES {
            match self.try_extract_port().await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    if attempt == MAX_RETRIES {
                        return Err(e);
                    }
                    println!("Failed to extract port (attempt {}), retrying...", attempt);
                    sleep(RETRY_DELAY).await;
                }
            }
        }

        Err(VirtualMachineError::Port)
    }

    async fn try_extract_port(&mut self) -> Result<(), VirtualMachineError> {
        let container_info = self.docker.inspect_container(&self.container_name, None).await?;
        let port = container_info.network_settings
            .and_then(|ns| ns.ports)
            .and_then(|ports| ports.get("3000/tcp").cloned())
            .and_then(|bindings| bindings)
            .and_then(|bindings| bindings.first().cloned())
            .and_then(|binding| binding.host_port)
            .ok_or(VirtualMachineError::Port)?;

        self.port = Some(port);

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        match self.docker.stop_container(&self.container_name, None).await {
            Ok(_) => println!("Container {} stopped", self.container_name),
            Err(e) => {
                if e.to_string().contains("container is not running") {
                    println!("Container {} was already stopped", self.container_name);
                } else {
                    eprintln!("Error stopping container {}: {}", self.container_name, e);
                }
            }
        }

        match self.docker.remove_container(&self.container_name, None).await {
            Ok(_) => println!("Container {} removed", self.container_name),
            Err(e) => return Err(Box::new(e)),
        }

        match self.docker.remove_network(&self.network_name).await {
            Ok(_) => println!("Network {} removed", self.network_name),
            Err(e) => eprintln!("Failed to remove network {}: {}", self.network_name, e),
        }

        println!("Container {} and associated resources cleaned up", self.container_name);

        Ok(())
    }
}
