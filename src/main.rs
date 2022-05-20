pub mod libs;

use crate::libs::utils::{draw_start_screen, load_config_toml};
use crate::libs::structs::TOMLData;

use log::{info, warn, error, debug};
use dotenv::dotenv;
use std::env;


fn main() {
    // Init environment vars and logger
    dotenv().ok();
    env_logger::init();

    draw_start_screen();
    let config_file = "notify.toml".to_string();

    let config: TOMLData = load_config_toml(config_file);
    debug!("{}",config.config.url)

}



