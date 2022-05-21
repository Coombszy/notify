pub mod libs;
use libs::structs::{Notification, TOMLData};
use libs::utils::*;

use dotenv::dotenv;
use log::debug;
use tokio_cron_scheduler::{Job, JobScheduler, JobToRun};

fn main() {
    startup();

    let data_folder: String = "data/".to_string();

    // Load TOML Data
    let toml_data: TOMLData = load_config_toml(format!("{}notify.toml", &data_folder));
    debug!("Config loaded:\n{}", toml_data.config.to_string_pretty());

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

// FOR FUTURE IMPL OF : https://crates.io/crates/tokio-cron-scheduler
// Creates scheduled
// #[tokio_cron_scheduler]
// async fn scheduler() {

// }
