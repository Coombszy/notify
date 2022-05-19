// Some utils to make life easier
use crate::libs::structs::{TOMLData};
use std::fs;
use std::process::exit;

pub fn draw_start_screen() {
    // Draws start screen containing app version and ascii
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

pub fn load_config_toml(filename: String) -> TOMLData {
    // Load TOMLData struct from filename
    // Load in raw string from config toml
    let toml_raw = match fs::read_to_string(&filename) {
        Ok(c) => c,
        // Failed to read file
        Err(_) => {
            eprintln!("could not read file '{}'", &filename);
            exit(1);
        }
    };

    let config_data: TOMLData = match toml::from_str(&toml_raw) {
        Ok(d) => d,
        // Failed to parse from String to TOMLData Struct
        Err(_) => {
            eprintln!("Unable to load data from {}", &filename);
            exit(1);
        }
    };

    return config_data;
}
