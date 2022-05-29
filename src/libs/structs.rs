// Struct storage
use serde::{Deserialize, Serialize};

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
    pub enabled: bool,
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

    // Returns body for IFTTTWebhook integration
    pub fn ifttt_body(&self) -> IftttBody {
        if self.image.is_none() {
            IftttBody {
                value1: self.title.clone(),
                value2: self.content.clone(),
                value3: None.clone(),
            }
        } else {
            IftttBody {
                value1: self.title.clone(),
                value2: self.title.clone(),
                value3: self.image.clone(),
            }
        }
    }
}

#[derive(Serialize)]
pub struct IftttBody {
    pub value1: String,
    pub value2: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value3: Option<String>,
}
