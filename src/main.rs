pub mod libs;
pub mod mocks;
pub mod tests;
use libs::structs::{Config, Notification, State, TOMLData};
use libs::utils::{draw_start_screen, load_config_toml, load_notifications};

// Compile using mocked IFTTT Sending
#[cfg(not(debug_assertions))]
use libs::utils::send_notification;
#[cfg(debug_assertions)]
use mocks::utils::send_notification;

use actix_web::{web, App, HttpServer};
use chrono::Utc;
use cronjob::CronJob;
use dotenv::dotenv;
use log::{debug, error, info, warn};
use std::process::exit;
use std::thread;
use std::time::Duration;

const DATA_FOLDER: &str = "config/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    startup();

    // Load TOML Data
    let toml_data: TOMLData = load_config_toml(format!("{}notify.toml", &DATA_FOLDER));
    debug!("Config loaded:\n{}", toml_data.config.display_pretty());

    if !(toml_data.config.schedule_enabled) && !(toml_data.config.web_enabled) {
        error!("Scheduled and Web notifications cannot both be disabled!");
        exit(1);
    }

    // Load Notifications
    if toml_data.config.schedule_enabled {
        let notifications: Vec<Notification> =
            load_notifications(&toml_data.config.schedule_source);
        info!("Notifications loaded: {}", notifications.len());

        // Create scheduled notifications
        notification_scheduler(&notifications, toml_data.config.clone());
    }

    // Start Actix Web
    if toml_data.config.web_enabled {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(State {
                    start_time: Utc::now(),
                    key: toml_data.clone().config.ifttt_key,
                    event: toml_data.clone().config.ifttt_event,
                }))
                .service(libs::routes::health)
                .service(libs::routes::notification)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    }
    // Keep app alive
    else {
        loop {
            thread::sleep(Duration::from_secs(10));
        }
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
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        handle.spawn(async move {
            send_notification(notification).await;
        });
        // Keep thread alive for so that Runtime is still alive to handle task
        // This feels like a bit of a hack but i cannot work out how to keep the runtime alive,
        // or how to pass a 'main'/global runtime into this command.
        thread::sleep(Duration::from_secs(15));
    }

    for notification in notifications {
        if !notification.enabled.unwrap() {
            continue;
        }
        // I hate this implementation, but seems the only way to insert data into the CronJob.
        // The Crate only allows parsing a 'name' into the function in the schedule. So, we
        // are squeezing in a JSON as the name so it can be deserialized on the other end.
        let mut _notification: Notification = notification.clone();
        _notification.key = Some(config.ifttt_key.clone());
        _notification.event = Some(config.ifttt_event.clone());
        let mut cron_job = CronJob::new(&serde_json::to_string(&_notification).unwrap(), cron_job);

        let unwrapped_cron = _notification.cron.unwrap();
        let cron: Vec<&str> = unwrapped_cron.split(' ').collect();

        if cron.len() != 5 {
            error!(
                "Cron formatting for \"{}\" was invalid. It should contain 5 fields!",
                &_notification.title
            );
            exit(1);
        }
        if cron[0] == "*" {
            warn!(
                "Seconds is less than or equal to 15 for \"{}\". Due to a runtime bug/issue, this cron will only run a maximum of 1 per 15 seconds.", &_notification.title
            )
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
