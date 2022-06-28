use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

// TOML Data on loaded on startup
#[derive(Deserialize, Clone)]
pub struct TOMLData {
    pub config: Config,
}

// Config data stored within TOML Data
#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub ifttt_key: String,
    pub ifttt_event: String,
    pub schedule_enabled: bool,
    pub schedule_source: String,
    pub web_enabled: bool,
    pub web_host: String,
    pub web_port: u16,
    pub write_logs: bool,
    pub write_logs_file: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
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
                value2: self.content.clone(),
                value3: self.image.clone(),
            }
        }
    }
}

// JSON body for outbound IFTTT WebHook requests
#[derive(Serialize)]
pub struct IftttBody {
    pub value1: String,
    pub value2: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value3: Option<String>,
}

// Actix Application global state
pub struct State {
    pub start_time: DateTime<Utc>,
    pub key: String,
    pub event: String,
}

// Global state impls
impl State {
    // Returns current uptime using `start_time`
    pub fn uptime(&self) -> String {
        let duration: Duration = Utc::now() - self.start_time;

        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        return format!("{hours:02}:{minutes:02}:{seconds:02}",);
    }
}

// Web reoute 'health' response body
#[derive(Serialize)]
pub struct WebHealth {
    pub uptime: String,
}

// Reponse error
#[derive(Serialize)]
pub struct WebError {
    pub timestamp: String,
    pub error: String,
}

/*
########################################################################################################
#   Copyright (C) 2022 Coombszy
#
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/