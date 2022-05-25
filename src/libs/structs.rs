// Struct storage
use serde::{Deserialize, Serialize};

// TOML Data on loaded on startup
#[derive(Deserialize)]
pub struct TOMLData {
    pub config: Config,
}

// Config data stored within TOML Data
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub wait: u8,
    pub scheduled_notifications: String,
    pub write_logs: bool,
    pub write_logs_dir: String,
}

// Implement functionality for Config struct
impl Config {
    // Returns struct as JSON
    pub fn display(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Returns struct as pretty JSON
    pub fn display_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

// Stores notification loaded from notifications.json
#[derive(Deserialize, Serialize)]
pub struct Notification {
    pub title: String,
    pub content: String,
    pub image: Option<String>,
    pub cron: String,
}

// Implement functionality for Notification struct
impl Notification {
    // Returns struct as JSON
    pub fn display(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Returns struct as pretty JSON
    pub fn display_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
