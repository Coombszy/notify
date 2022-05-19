pub mod libs;
use crate::libs::utils::{draw_start_screen, load_config_toml};
use crate::libs::structs::TOMLData;


fn main() {
    draw_start_screen();
    let config_file = "notify.toml".to_string();

    let config: TOMLData = load_config_toml(config_file);
    println!("{}",config.config.url)

}



