// Some utils to make life easier
use crate::libs::structs::{Notification, TOMLData};
use log::error;
use std::fs;
use std::process::exit;

// Draws start screen containing app version and ascii
pub fn draw_start_screen() {
    let ascii_name = r#"     _   _       _   _  __       
    | \ | |     | | (_)/ _|      
    |  \| | ___ | |_ _| |_ _   _ 
    | . ` |/ _ \| __| |  _| | | |
    | |\  | (_) | |_| | | | |_| |
    |_| \_|\___/ \__|_|_|  \__, |
                            __/ |
                            |___/ "#;

    println!("{} v{}", &ascii_name, &env!("CARGO_PKG_VERSION"));
    println!("\n   Created by {}", &env!("CARGO_PKG_AUTHORS"));
    println!("==============================================")
}

// Loads TOMLData struct from filename
pub fn load_config_toml(filename: String) -> TOMLData {
    // Load in raw string from config toml
    let toml_raw = match fs::read_to_string(&filename) {
        Ok(c) => c,
        // Failed to read file
        Err(e) => {
            error!("Could not read TOML file '{}'", &filename);
            error!("Error: {}", e);
            exit(1);
        }
    };
    // Convert to TOML struct
    let config_data: TOMLData = match toml::from_str(&toml_raw) {
        Ok(d) => d,
        // Failed to parse from String to TOMLData Struct
        Err(e) => {
            error!("Unable to load data from {}", &filename);
            error!("Error: {}", e);
            exit(1);
        }
    };
    return config_data;
}

// Loads Notification Vector from filename
pub fn load_notifications(filename: String) -> Vec<Notification> {
    // Load raw json into string
    let json_raw = match fs::read_to_string(&filename) {
        Ok(c) => c,
        // Failed to read file
        Err(e) => {
            error!("Could not read JSON file '{}'", &filename);
            error!("Error: {}", e);
            exit(1);
        }
    };
    // Convert to Vector Nofitication Struct
    let nofitication_data: Vec<Notification> = match serde_json::from_str(&json_raw) {
        Ok(d) => d,
        // Failed to parse to from String to Vec<Notifications Struct>
        Err(e) => {
            error!("Unable to load data from {}", &filename);
            error!("Error: {}", e);
            exit(1);
        }
    };
    return nofitication_data;
}
