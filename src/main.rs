pub mod libs;
use crate::libs::utils::draw_start_screen;
use crate::libs::structs::{TOMLData, Config};
use std::fs;
use std::process::exit;

fn main() {
    draw_start_screen();
    let config_file = "notify.toml";

    // Load in raw string from config toml
    let toml_raw = match fs::read_to_string(config_file) {
        Ok(c) => c,
        // Failed to read file
        Err(_) => {
            eprintln!("could not read file '{}'", config_file);
            exit(1);
        }
    };

    let config_data: TOMLData = match toml::from_str(&toml_raw) {
        Ok(d) => d,
        // Failed to parse from String to TOMLData Struct
        Err(_) => {
            eprintln!("Unable to load data from {}", config_file);
            exit(1);
        }
    };
}
