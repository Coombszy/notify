// Struct storage
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TOMLData {
    pub config: Config
}

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
    pub wait: u8,
    pub scheduled_notifications: String,
    pub write_logs: bool,
    pub write_logs_dir: String
}
