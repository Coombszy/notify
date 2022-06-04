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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    startup();

    let data_folder: String = "data/".to_string();

    // Load TOML Data
    let toml_data: TOMLData = load_config_toml(format!("{}notify.toml", &data_folder));
    debug!("Config loaded:\n{}", toml_data.config.display_pretty());

    // Load Notifications
    let notifications: Vec<Notification> =
        load_notifications(format!("{}notifications.json", &data_folder));
    info!("Notifications loaded: {}", notifications.len());

    // Create scheduled notifications
    notification_scheduler(&notifications, toml_data.config);

    // Start Actix Web
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(State {
                start_time: Utc::now(),
            }))
            .service(libs::routes::health)
            .service(libs::routes::notification)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
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
        if !notification.enabled {
            continue;
        }
        // I hate this implementation, but seems the only way to insert data into the CronJob.
        // The Crate only allows parsing a 'name' into the function in the schedule. So, we
        // are squeezing in a JSON as the name so it can be deserialized on the other end.
        let mut _notification: Notification = notification.clone();
        _notification.key = Some(config.key.clone());
        _notification.event = Some(config.event.clone());
        let mut cron_job = CronJob::new(&serde_json::to_string(&_notification).unwrap(), cron_job);

        let cron: Vec<&str> = _notification.cron.split(' ').collect();

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
