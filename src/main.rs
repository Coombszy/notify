pub mod tests;
pub mod libs;
use libs::structs::{Notification, TOMLData, Config};
use libs::utils::*;

use dotenv::dotenv;
use log::{error, debug, info};
use cronjob::CronJob;
use ifttt_webhook::IftttWebhook;
use std::thread;
use std::time::Duration;
use std::process::exit;


fn main() {
    startup();

    let data_folder: String = "data/".to_string();

    // Load TOML Data
    let toml_data: TOMLData = load_config_toml(format!("{}notify.toml", &data_folder));
    debug!("Config loaded:\n{}", toml_data.config.display_pretty());

    // Load Notifications
    let notifications: Vec<Notification> =
        load_notifications(format!("{}notifications.json", &data_folder));
    info!("Notifications loaded: {}", notifications.len());

    notification_scheduler(&notifications, toml_data.config);

    // Not sure if this will be needed with Web servera
    // Remove me for final version as this _should_ not be needed
    loop{
        thread::sleep(Duration::from_secs(10));
        println!("                                                      ----- CORE LOOP -----");
    }
}

// Executes basic startup functions
fn startup() {
    // Init environment vars and logger
    dotenv().ok();
    env_logger::init();

    draw_start_screen();
}

// Creates CronJobs on new threads with notifications list
fn notification_scheduler(notifications: &Vec<Notification>, config: Config) {
    fn cron_job(data: &str) {
        let notification: Notification = serde_json::from_str(data).unwrap();

        let _webhook = IftttWebhook {
            key: notification.key.clone().unwrap(),
            event: notification.event.clone().unwrap(),
        };
        
        // Currently broken
        let _ = &notification.to_ifttt_hashmap();
        //webhook.trigger(Some(&notification.to_ifttt_hashmap()));
        
        
        debug!("tempTrigger (title): {}", notification.title);
    }

    for notification in notifications {
        // I hate this implementation, but seems the only way to insert data into the CronJob.
        // The Crate only allows parsing a 'name' into the function in the schedule. So, we 
        // are squeezing in a JSON as the name so it can be deserialized on the other end.
        let mut _notification: Notification = notification.clone();
        _notification.key = Some(config.key.clone());
        _notification.event = Some(config.event.clone());
        let mut cron_job = CronJob::new(&serde_json::to_string(&_notification).unwrap(), cron_job);

        let cron: Vec<&str> = _notification.cron.split(' ').collect();

        if cron.len() != 5 {
            error!("Cron formatting for \"{}\" was invalid. It should contain 5 fields!", &_notification.title);
            exit(1);
        }

        // Convert schedule to CronJob
        cron_job.seconds(cron[0]);
        cron_job.minutes(cron[1]);
        cron_job.hours(cron[2]);
        cron_job.day_of_month(cron[3]);
        cron_job.month(cron[4]);

        CronJob::start_job_threaded(cron_job);
    }
}
