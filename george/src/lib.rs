mod daemon;

use crate::daemon::{Daemon, DaemonError};
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::image::BuildImageOptions;
use bollard::models::{HostConfig, PortBinding};
use bollard::network::CreateNetworkOptions;
use bollard::Docker;
use bytes::Bytes;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tar::Builder;
use uuid::Uuid;


pub struct George {
    docker: Docker,
    container_id: Option<String>,
    port: Option<String>,
    network_name: Option<String>, // Added network name field
    pub daemon: Daemon,
}

impl Default for George {
    fn default() -> Self {
        Self::new()
    }
}


impl George {
    pub fn new() -> Self {
        Self {
            docker: Docker::connect_with_local_defaults().expect("Failed to connect to Docker"),
            container_id: None,
            port: None,
            network_name: None,
            daemon: Daemon::new(),
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let unique_name = format!("george-{}", Uuid::new_v4());
        let image_name = format!("george-daemon:{}", unique_name);

        // Path to the directory containing the Dockerfile
        let dockerfile_path = Path::new("/Users/logankeenan/Developer/george/george-daemon");

        // Create a tar archive of the directory
        let tar_path = Path::new("/tmp/george-daemon.tar");
        let file = File::create(tar_path)?;
        let mut builder = Builder::new(file);
        builder.append_dir_all(".", dockerfile_path)?;
        builder.finish()?;


        let tar_contents = std::fs::read(tar_path)?;
        let build_options = BuildImageOptions::<&str> { dockerfile: "Dockerfile", t: image_name.as_str(), ..Default::default() };

        let mut build_stream = self.docker.build_image(
            build_options,
            None,
            Some(tar_contents.into()),
        );

        while let Some(build_result) = build_stream.next().await {
            match build_result {
                Ok(output) => println!("Build output: {:?}", output),
                Err(e) => return Err(format!("Build error: {:?}", e).into()),
            }
        }

        let network_name = format!("george-network-{}", Uuid::new_v4());
        self.network_name = Some(network_name.clone());
        self.docker.create_network(CreateNetworkOptions {
            name: network_name.as_str(),
            ..Default::default()
        }).await?;

        // Create the container
        let mut exposed_ports = HashMap::new();
        exposed_ports.insert(String::from("3000/tcp"), HashMap::new());

        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            String::from("3000/tcp"),
            Some(vec![PortBinding {
                host_ip: Some(String::from("0.0.0.0")),
                host_port: None, // Change this to None for dynamic port allocation
            }]),
        );

        let host_config = HostConfig {
            port_bindings: Some(port_bindings),
            network_mode: Some(network_name.clone()),
            ..Default::default()
        };

        let container = self.docker.create_container(
            Some(CreateContainerOptions { name: unique_name.clone(), platform: None }),
            Config {
                image: Some(image_name),
                exposed_ports: Some(exposed_ports),
                host_config: Some(host_config),
                env: Some(vec!["DISPLAY=:99".to_string()]),
                cmd: Some(vec![
                    String::from("sh"), String::from("-c"),
                    String::from("Xvfb :99 -screen 0 1024x768x16 & sleep 2 && firefox http://host.docker.internal:3001 --width=1024 --height=768 --display=:99 & sleep 5 && ./george-daemon")
                ]),
                ..Default::default()
            },
        ).await?;

        // Start the container
        self.docker.start_container(&container.id, None::<StartContainerOptions<String>>).await?;

        // Get the dynamically assigned port
        let container_info = self.docker.inspect_container(&container.id, None).await?;
        let port = container_info.network_settings
            .and_then(|ns| ns.ports)
            .and_then(|ports| ports.get("3000/tcp").cloned())
            .and_then(|bindings| bindings)
            .and_then(|bindings| bindings.first().cloned())
            .and_then(|binding| binding.host_port)
            .ok_or("Failed to get dynamic port")?;

        self.container_id = Some(container.id);
        println!("running on port: {:?}", port);
        self.daemon.set_port(port.clone());
        self.port = Some(port);


        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(container_id) = self.container_id.take() {
            // Stop the container
            match self.docker.stop_container(&container_id, None).await {
                Ok(_) => println!("Container {} stopped", container_id),
                Err(e) => {
                    if e.to_string().contains("container is not running") {
                        println!("Container {} was already stopped", container_id);
                    } else {
                        eprintln!("Error stopping container {}: {}", container_id, e);
                    }
                }
            }

            // Remove the container
            match self.docker.remove_container(&container_id, None).await {
                Ok(_) => println!("Container {} removed", container_id),
                Err(e) => return Err(Box::new(e)),
            }

            // Get the network name and remove it
            if let Some(network_name) = self.network_name.take() {
                match self.docker.remove_network(&network_name).await {
                    Ok(_) => println!("Network {} removed", network_name),
                    Err(e) => eprintln!("Failed to remove network {}: {}", network_name, e),
                }
            }

            println!("Container {} and associated resources cleaned up", container_id);
        } else {
            println!("No container to stop");
        }

        Ok(())
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