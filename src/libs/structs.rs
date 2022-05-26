// Struct storage
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TOML Data on loaded on startup
#[derive(Deserialize)]
pub struct TOMLData {
    pub config: Config,
}

// Config data stored within TOML Data
#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub key: String,
    pub event: String,
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
#[derive(Deserialize, Serialize, Clone)]
pub struct Notification {
    pub title: String,
    pub content: String,
    pub image: Option<String>,
    pub cron: String,
    pub event: Option<String>,
    pub key: Option<String>,
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

    // Returns Hashmap for IFTTTWebhook integration
    pub fn to_ifttt_hashmap(&self) -> HashMap<String, String> {
        let mut ifttt_hashmap = HashMap::new();
        ifttt_hashmap.insert("value1".to_string(), self.title.to_string());
        ifttt_hashmap.insert("value2".to_string(), self.content.to_string());
        if self.image.is_some() {
            ifttt_hashmap.insert("value3".to_string(), self.image.clone().unwrap().to_string());
        }
        return ifttt_hashmap;
    }
}
