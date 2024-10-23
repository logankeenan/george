use std::error::Error;
use serde_json::json;
use reqwest::{Client, Response};
use bytes::Bytes;
use serde::Deserialize;
use base64::{engine::general_purpose, Engine as _};
use image::ImageFormat;
use image::ImageReader;

pub struct George {}

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

impl Default for George {
    fn default() -> Self {
        Self::new()
    }
}


impl George {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn click_coordinate(&self, x: u32, y: u32) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let url = "http://127.0.0.1:3000/click";
        let body = json!({
            "x": x,
            "y": y,
        });
        let res = client.post(url)
            .json(&body)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let status = res.status();
            let text = res.text().await?;
            Err(format!("Failed to send click: Status: {}, Body: {}", status, text).into())
        }
    }

    pub async fn click(&self, selector: &str) -> Result<(), Box<dyn std::error::Error>> {
        let coordinate = self.coordinate_of(selector).await?;

        self.click_coordinate(coordinate.0, coordinate.1).await
    }


    pub async fn screenshot(&self) -> Result<Bytes, Box<dyn Error>> {
        let client = Client::new();
        let response: Response = client
            .get("http://localhost:3000/screenshot")
            .send()
            .await?;

        if response.status().is_success() {
            let image_bytes = response.bytes().await?;
            Ok(image_bytes)
        } else {
            Err(Box::from("Failed to retrieve screenshot."))
        }
    }

    pub async fn coordinate_of(&self, selector: &str) -> Result<(u32, u32), Box<dyn Error>> {
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

        let client = Client::new();
        let response = client
            .post("http://logan-server:8000/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::from("Failed to process the find request."));
        }

        let response_body: FindResponse = response.json().await?;
        let content = response_body.choices.first()
            .ok_or("No choices in response")?
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
            Err(Box::from("Failed to parse coordinates from response."))
        }
    }

    pub async fn fill_in(&self, selector: &str, with: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.click(selector).await?;

        self.type_text(with).await
    }

    pub async fn type_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let url = "http://127.0.0.1:3000/type";
        let body = json!({
            "text": text,
        });

        let res = client.post(url)
            .json(&body)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let status = res.status();
            let response_text = res.text().await?;
            Err(format!("Failed to send text: Status: {}, Body: {}", status, response_text).into())
        }
    }
}