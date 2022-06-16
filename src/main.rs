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
use log::{debug, error, info};
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::env;
use std::fs::File;
use std::process::exit;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use time::macros::format_description;

const DATA_FOLDER: &str = "config/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let toml_data: TOMLData = startup();

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
        let host: String = toml_data.clone().config.web_host;
        let port: u16 = toml_data.clone().config.web_port;
        info!("Starting web server, listening on {host}:{port}");
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
        .bind((host, port))?
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
fn startup() -> TOMLData {
    draw_start_screen();

    // Init environment vars from .env file
    dotenv().ok();

    // Load TOML Data for config
    let toml_data: TOMLData = load_config_toml(format!("{}notify.toml", &DATA_FOLDER));

    // Init logging
    // Is NOTIFY_LOG_LEVEL in environment vars
    let level: LevelFilter = if env::var("NOTIFY_LOG_LEVEL").is_err() {
        LevelFilter::Info
    } else {
        LevelFilter::from_str(env::var("NOTIFY_LOG_LEVEL").unwrap().as_str()).unwrap()
    };
    // Create custom config
    let mut config: ConfigBuilder = simplelog::ConfigBuilder::default();
    config.set_time_format_custom(format_description!(
        "[hour]:[minute]:[second] [day]/[month]/[year]"
    ));
    if toml_data.config.write_logs {
        CombinedLogger::init(vec![
            TermLogger::new(
                level,
                config.build(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                level,
                config.build(),
                File::create(toml_data.config.write_logs_file.clone()).unwrap(),
            ),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![TermLogger::new(
            level,
            config.build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    }

    // Config validation
    debug!("Config loaded:\n{}", toml_data.config.display_pretty());
    // Do not start with Scheduled and Web disabled
    if !(toml_data.config.schedule_enabled) && !(toml_data.config.web_enabled) {
        error!("Scheduled and Web notifications cannot both be disabled!");
        exit(1);
    }

    toml_data
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
        thread::sleep(Duration::from_secs(30));
    }

    for notification in notifications {
        if !notification.enabled.unwrap() {
            continue;
        }
        // I hate this implementation, but seems the only way to insert data into the CronJob.
        // The Crate only allows parsing a 'name' into the function in the schedule. So, we
        // are squeezing in a JSON as the name so it can be deserialized on the other end.
        let mut _notification: Notification = notification.clone();
        debug!("Creating notification cron:\n{}", _notification.display_pretty());
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

        // Convert schedule to CronJob
        cron_job.seconds("0");
        cron_job.minutes(cron[0]);
        cron_job.hours(cron[1]);
        cron_job.day_of_month(cron[2]);
        cron_job.month(cron[3]);
        cron_job.day_of_week(cron[4]); // For some reason, the crate is off by one from the `usual` cron expression if you use integers? use wed, mon instead

        CronJob::start_job_threaded(cron_job);
    }
}
