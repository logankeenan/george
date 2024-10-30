use std::fs::{create_dir_all, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonOutputError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DaemonOutputEntry {
    pub id: Uuid,
    pub prompt: Option<String>,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    pub coordinates: Option<(u32, u32)>,
    pub screenshot: Option<String>,
    pub error: Option<String>,
}

pub struct DaemonOutput {
    uuid: String,
}

impl DaemonOutput {
    pub fn new(uuid: Uuid) -> Self {
        let uuid_str = uuid.to_string();
        Self {
            uuid: uuid_str,
        }
    }

    fn get_log_dir() -> PathBuf {
        let mut path = std::env::current_exe().expect("Failed to get current executable path");
        path.pop(); // Remove the executable name
        path.push(".george");
        path.push("logs");
        path
    }

    fn get_screenshots_dir() -> PathBuf {
        let mut path = std::env::current_exe().expect("Failed to get current executable path");
        path.pop(); // Remove the executable name
        path.push(".george");
        path.push("screenshots");
        path
    }


    fn get_file_path(&self) -> PathBuf {
        let mut path = Self::get_log_dir();
        path.push(format!("daemon-{}.json", self.uuid));
        path
    }

    fn read_log_entries(&self) -> Result<Vec<DaemonOutputEntry>, DaemonOutputError> {
        let file_path = self.get_file_path();
        let file = OpenOptions::new().read(true).open(&file_path);

        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let entries: Vec<DaemonOutputEntry> = serde_json::from_str(&contents)?;
                Ok(entries)
            }
            Err(_) => Ok(Vec::new()), // Return an empty vector if the file doesn't exist
        }
    }

    fn write_log_entries(&self, entries: &[DaemonOutputEntry]) -> Result<(), DaemonOutputError> {
        let file_path = self.get_file_path();
        create_dir_all(file_path.parent().unwrap())?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        let json = serde_json::to_string_pretty(entries)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn log(&self, entry: &mut DaemonOutputEntry) -> Result<(), DaemonOutputError> {
        let mut entries = self.read_log_entries()?;

        if let Some(existing_entry) = entries.iter_mut().find(|e| e.id == entry.id) {
            *existing_entry = entry.clone();
        } else {
            entries.push(entry.clone());
        }

        self.write_log_entries(&entries)
    }

    pub fn save_screenshot(&self, entry: &mut DaemonOutputEntry, screenshot: &[u8]) -> Result<(), DaemonOutputError> {
        let screenshots_dir = Self::get_screenshots_dir();
        create_dir_all(&screenshots_dir)?;
        let screenshot_filename = screenshots_dir.join(format!("screenshot-{}.png", entry.id));
        std::fs::write(&screenshot_filename, screenshot)?;

        entry.screenshot = Some(screenshot_filename.to_str().unwrap().to_string());
        self.log(entry)?;

        Ok(())
    }
}
