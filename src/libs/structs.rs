// Struct storage
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TOMLData {
    config: Config
}

#[derive(Deserialize)]
pub struct Config {
    url: String,
    wait: u8,
    scheduled_notifications: String,
    write_logs: bool,
    write_logs_dir: String
}