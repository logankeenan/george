use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use image::ImageFormat;
use image::ImageReader;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use regex::Regex;
use thiserror::{Error};
use tokio::time::{sleep, timeout};

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
    #[error("Failed to parse coordinates from: {0}")]
    FailedToParseCoordinates(String),
    #[error("Failed to parse existence from: {0}")]
    FailedToParseExistence(String),
    #[error("Screenshot failed: {0}")]
    ScreenshotFailed(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Timeout while trying to find selector: {0}")]
    SelectorTimeout(String),
}

#[derive(Deserialize, Serialize, Debug)]
struct FindResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    content: String,
}


pub struct Daemon {
    port: Option<String>,
    client: Client,
    pub settings: DaemonSettings,
}

#[derive(Clone, Debug)]
pub struct DaemonSettings {
    vision_coordinate_prompt: String,
    vision_llm_url: String,
    is_text_visible_prompt: String,
}

impl DaemonSettings {
    pub fn new(vision_llm_url: &str) -> Self {
        Self {
            vision_coordinate_prompt: String::from("You are a helpful assistant that is to be used in finding coordinates of items in an image. You are finding coordinates so you can be part of a automated AI tool. You need to be as accurate as possible. Find the point coordinate of the center of the "),
            is_text_visible_prompt: String::from("find all the text on the screen. return it in an array list"),
            vision_llm_url: vision_llm_url.to_string(),
        }
    }

    pub fn set_vision_coordinate_prompt(mut self, vision_coordinate_prompt: String) -> Self {
        self.vision_coordinate_prompt = vision_coordinate_prompt;
        self
    }

    pub fn set_is_text_visible_prompt(mut self, is_text_visible_prompt: String) -> Self {
        self.is_text_visible_prompt = is_text_visible_prompt;
        self
    }
}


impl Daemon {
    pub fn new(visual_llm_url: &str) -> Self {
        Self {
            port: None,
            client: Client::new(),
            settings: DaemonSettings::new(visual_llm_url),
        }
    }

    pub fn with_settings(settings: DaemonSettings) -> Self {
        Self {
            port: None,
            client: Client::new(),
            settings,
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
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("Unable to retrieve response body"));
            Err(DaemonError::ScreenshotFailed(format!(
                "Failed to retrieve screenshot. Status: {}, Body: {}",
                status, body
            )))
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

    pub async fn ready(&self) -> Result<(), DaemonError> {
        let timeout_duration = Duration::from_secs(10);

        timeout(timeout_duration, async {
            loop {
                match self.check_health().await {
                    Ok(()) => return Ok(()),
                    Err(_) => sleep(Duration::from_millis(100)).await,
                }
            }
        })
            .await
            .map_err(|_| DaemonError::Unexpected("Daemon health check timed out after 10 seconds".into()))?
    }

    async fn check_health(&self) -> Result<(), DaemonError> {
        let url = self.build_url("healthz")?;
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(DaemonError::Unexpected("Daemon health check failed".into()))
        }
    }

    pub async fn is_text_visible_from_prompt(&self, prompt: &str) -> Result<Vec<String>, DaemonError> {
        println!();
        println!("prompt: {}", prompt);
        let screenshot_bytes = self.screenshot().await?;

        let image_base64 = general_purpose::STANDARD.encode(&screenshot_bytes);

        let request_body = json!({
            "model": "allenai/Molmo-7B-D-0924",
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {"type": "text", "text": prompt},
                        {"type": "image_url", "image_url": {"url": format!("data:image/jpeg;base64,{}", image_base64)}}
                    ]
                }
            ],
            "temperature": 0,
            "top_k": 1
        });

        let response = self.client
            .post(format!("{}/v1/chat/completions", self.settings.vision_llm_url))
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DaemonError::Unexpected("Failed to process the existence check request.".into()));
        }

        let response_body: FindResponse = response.json().await?;
        let response_text = serde_json::to_string(&response_body)?;
        println!("Full response body: {}", response_text);

        let content = response_body.choices.first()
            .ok_or_else(|| DaemonError::Unexpected(format!("No choices in response. Prompt: {}", prompt)))?
            .message.content.trim().to_lowercase();


        println!("content: {:?}", content);
        let visible_text = self.parse_visible_text(content.as_str())?;
        Ok(visible_text)
    }

    pub async fn is_text_visible(&self, text: &str) -> Result<bool, DaemonError> {
        let visible_texts = self.is_text_visible_from_prompt(self.settings.is_text_visible_prompt.as_str()).await?;
        let lowercase_text = text.to_lowercase();

        Ok(visible_texts.iter().any(|t| t.to_lowercase() == lowercase_text))
    }


    pub(crate) fn parse_visible_text(&self, content: &str) -> Result<Vec<String>, DaemonError> {
        let re = Regex::new(r#""(.*?)""#).unwrap(); // Matches text within double quotes
        let mut visible_text = Vec::new();


        for cap in re.captures_iter(content) {
            if let Some(text) = cap.get(1) {
                visible_text.push(text.as_str().trim_end_matches('\\').to_string());
            }
        }

        if visible_text.is_empty() {
            return Err(DaemonError::Unexpected("No visible text found".to_string()));
        }

        Ok(visible_text)
    }

    pub async fn coordinate_of_from_prompt(&self, prompt: &str) -> Result<(u32, u32), DaemonError> {
        println!();
        println!("prompt: {}", prompt);
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
                        {"type": "text", "text": prompt},
                        {"type": "image_url", "image_url": {"url": format!("data:image/jpeg;base64,{}", image_base64)}}
                    ]
                }
            ],
            "temperature": 0,
            "top_k": 1
        });

        let response = self.client
            .post(format!("{}/v1/chat/completions", self.settings.vision_llm_url))
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DaemonError::Unexpected("Failed to process the find request.".into()));
        }

        let response_body: FindResponse = response.json().await?;
        let response_text = serde_json::to_string(&response_body)?;
        println!("Full response body: {}", response_text);

        let content = response_body.choices.first()
            .ok_or_else(|| DaemonError::Unexpected(format!("No choices in response. Prompt: {}", prompt)))?
            .message.content.trim();

        let parsed_coords = self.parse_coordinates(content)?;

        let pixel_coordinates = self.calculate_coordinates(parsed_coords, width, height);
        println!("pixel_coordinates: {:?}", pixel_coordinates);
        println!();
        pixel_coordinates
    }

    pub async fn coordinate_of(&self, selector: &str) -> Result<(u32, u32), DaemonError> {
        let prompt = format!("{} {}", self.settings.vision_coordinate_prompt, selector);
        self.coordinate_of_from_prompt(&prompt).await
    }

    fn parse_coordinates(&self, content: &str) -> Result<(f64, f64), DaemonError> {
        let re_xml = Regex::new(r#"x\d*="\s*([0-9]+(?:\.[0-9]+)?)"\s+y\d*="\s*([0-9]+(?:\.[0-9]+)?)"#).unwrap();
        let re_parens = Regex::new(r#"\(?\s*(\d+(?:\.\d+)?)\s*,\s*(\d+(?:\.\d+)?)\s*\)?"#).unwrap();
        let mut all_points = Vec::new();


        for cap in re_xml.captures_iter(content) {
            if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                if let (Ok(x), Ok(y)) = (x.as_str().parse::<f64>(), y.as_str().parse::<f64>()) {
                    if x <= 100.0 && y <= 100.0 {
                        all_points.push((x, y));
                    }
                }
            }
        }

        for cap in re_parens.captures_iter(content) {
            if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                if let (Ok(x), Ok(y)) = (x.as_str().parse::<f64>(), y.as_str().parse::<f64>()) {
                    if x <= 100.0 && y <= 100.0 {
                        all_points.push((x, y));
                    }
                }
            }
        }


        if all_points.is_empty() {
            Err(DaemonError::FailedToParseCoordinates(String::from(content)))
        } else {
            let option = *all_points.first().unwrap();
            Ok(option)
        }
    }

    fn calculate_coordinates(&self, coords: (f64, f64), width: u32, height: u32) -> Result<(u32, u32), DaemonError> {
        let x = ((coords.0 / 100.0) * width as f64) as u32;
        let y = ((coords.1 / 100.0) * height as f64) as u32;
        Ok((x, y))
    }

    pub async fn click(&self, selector: &str) -> Result<(), DaemonError> {
        let coordinate = self.coordinate_of(selector).await?;
        self.click_coordinate(coordinate.0, coordinate.1).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinates() {
        let daemon = Daemon::new("https://doesnotmatter.com");
        let input = r#"<points x1="0.6" y1="13.0" x2="104.7" y2="12.8" alt="the point coordinate of the sign up name input field">the point coordinate of the sign up name input field</points>"#;
        let result = daemon.parse_coordinates(input).unwrap();
        assert_eq!(result, (0.6, 13.0));
    }

    #[test]
    fn test_parse_coordinates_in_parens() {
        let daemon = Daemon::new("https://doesnotmatter.com");
        let input = r#"The center of the name input field is at coordinates (10.9, 14.1) in the image. This point represents the midpoint of the horizontal rectangle that contains the input field for the user's name.""#;
        let result = daemon.parse_coordinates(input).unwrap();
        assert_eq!(result, (10.9, 14.1));
    }

    #[test]
    fn test_parse_the_visible_text() {
        let daemon = Daemon::new("https://doesnotmatter.com");
        let input = r#" Here's an array list containing all the text visible on the screen:\n\n[\n  \"host.docker.internal:3001\",\n  \"Firefox Privacy Notice\",\n  \"End-to-End Test\",\n  \"Name\",\n  \"Phone\",\n  \"Email\",\n  \"First Programmer\",\n  \"Analytical Engine\",\n  \"Programming\",\n  \"Submit\"\n]\n\nThis list includes all the text elements present in the browser tab, address bar, form fields, and buttons visible in the image."#;
        let result = daemon.parse_visible_text(input).unwrap();
        assert_eq!(result, vec![
            "host.docker.internal:3001",
            "Firefox Privacy Notice",
            "End-to-End Test",
            "Name",
            "Phone",
            "Email",
            "First Programmer",
            "Analytical Engine",
            "Programming",
            "Submit"
        ]);
    }


}


