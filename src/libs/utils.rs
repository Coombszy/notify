// Some utils to make life easier

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
