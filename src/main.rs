pub mod tests;
pub mod libs;
use libs::structs::{Notification, TOMLData};
use libs::utils::*;

use dotenv::dotenv;
use log::debug;

fn main() {
    startup();

    let data_folder: String = "data/".to_string();

    // Load TOML Data
    let toml_data: TOMLData = load_config_toml(format!("{}notify.toml", &data_folder));
    debug!("Config loaded:\n{}", toml_data.config.display_pretty());

    // Load Notifications
    let notifications: Vec<Notification> =
        load_notifications(format!("{}notifications.json", &data_folder));
    debug!("Notifications loaded: {}", notifications.len());
}

// Executes basic startup functions
fn startup() {
    // Init environment vars and logger
    dotenv().ok();
    env_logger::init();

    draw_start_screen();
}

// TODO: https://docs.rs/tokio/1.18.2/tokio/attr.main.html AND https://crates.io/crates/tokio-cron-scheduler
// Creates scheduled
// async fn scheduler() {
// }
