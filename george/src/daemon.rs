use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use image::ImageFormat;
use image::ImageReader;
use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::json;
use std::default::Default;
use thiserror::{Error};

#[derive(Error, Debug)]
pub enum DaemonError {
    #[error("Daemon not started")]
    NotStarted,
    #[error("Failed to send request: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Image decoding failed: {0}")]
    ImageDecodingError(#[from] image::ImageError),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[derive(Deserialize, Debug)]
struct FindResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Message {
    content: String,
}


#[derive(Default)]
pub struct Daemon {
    port: Option<String>,
    client: Client,
}

impl Daemon {
    pub fn new() -> Self {
        Self {
            port: None,
            client: Client::new(),
        }
    }

    pub fn set_port(&mut self, port: String) {
        self.port = Some(port);
    }

    fn build_url(&self, endpoint: &str) -> Result<String, DaemonError> {
        let port = self.port.as_ref().ok_or(DaemonError::NotStarted)?;
        Ok(format!("http://127.0.0.1:{}/{}", port, endpoint))
    }

    async fn click_coordinate(&self, x: u32, y: u32) -> Result<(), DaemonError> {
        let url = self.build_url("click")?;
        let body = json!({
            "x": x,
            "y": y,
        });
        let res = self.client.post(url)
            .json(&body)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let status = res.status();
            let text = res.text().await?;
            Err(DaemonError::Unexpected(format!(
                "Failed to send click: Status: {}, Body: {}",
                status, text
            )))
        }
    }

    pub async fn screenshot(&self) -> Result<Bytes, DaemonError> {
        let url = self.build_url("screenshot")?;

        let response: Response = self.client
            .get(url)
            .send()
            .await?;

        if response.status().is_success() {
            let image_bytes = response.bytes().await?;
            Ok(image_bytes)
        } else {
            Err(DaemonError::Unexpected("Failed to retrieve screenshot.".into()))
        }
    }

    pub async fn type_text(&self, text: &str) -> Result<(), DaemonError> {
        let res = self.client.post(self.build_url("type")?)
            .json(&json!({
                "text": text,
            }))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let status = res.status();
            let response_text = res.text().await?;
            Err(DaemonError::Unexpected(format!(
                "Failed to send text: Status: {}, Body: {}",
                status, response_text
            )))
        }
    }

    pub async fn coordinate_of(&self, selector: &str) -> Result<(u32, u32), DaemonError> {
        let screenshot_bytes = self.screenshot().await?;

        let img = ImageReader::with_format(std::io::Cursor::new(&screenshot_bytes), ImageFormat::Png)
            .decode()?;
        let (width, height) = (img.width(), img.height());
        let image_base64 = general_purpose::STANDARD.encode(&screenshot_bytes);

        let request_body = json!({
            "model": "allenai/Molmo-7B-D-0924",
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {"type": "text", "text": format!("Find the {} and return the coordinates. The response should only be in the follow format: [x, y]", selector)},
                        {"type": "image_url", "image_url": {"url": format!("data:image/jpeg;base64,{}", image_base64)}}
                    ]
                }
            ]
        });

        let response = self.client
            .post("http://logan-server:8000/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DaemonError::Unexpected("Failed to process the find request.".into()));
        }

        let response_body: FindResponse = response.json().await?;

        let content = response_body.choices.first()
            .ok_or_else(|| DaemonError::Unexpected(format!("No choices in response. Selector: {}", selector)))?
            .message.content.trim();

        let coords: Vec<f64> = content
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().parse().unwrap_or(0.0))
            .collect();


        if coords.len() == 2 {
            let x = ((coords[0] / 100.0) * width as f64) as u32;
            let y = ((coords[1] / 100.0) * height as f64) as u32;
            Ok((x, y))
        } else {
            Err(DaemonError::Unexpected(format!("Failed to parse coordinates \n response: {:?} \n selector: {}", response_body, selector)))
        }
    }

    pub async fn click(&self, selector: &str) -> Result<(), DaemonError> {
        let coordinate = self.coordinate_of(selector).await?;
        self.click_coordinate(coordinate.0, coordinate.1).await
    }
}