use std::fs::File;
use std::io::prelude::*;
use std::{ thread, time::Duration };

use serde_derive::Deserialize;

mod server_manager;
mod discord;
mod server_tasks;

use server_manager::*;
use discord::*;

#[derive(Deserialize)]
struct Config {
    discord_url: Option<String>,
    server_directory: Option<String>,
    _restart_time: Option<String>
}

fn main() {
    let config = read_config_file("mcmanager.toml");

    let webhook: Option<DiscordWebhook> = match config.discord_url {
        Some(s) => Some(DiscordWebhook::new(&s)),
        None => None
    };
    
    // Set up the server
    let mut server = Manager::new();
    if let Some(s) = config.server_directory {
        server.server_directory(&s);
    }

    server.start();

    let action = |line: String| {
        match &webhook {
            Some(w) => w.post_string(&line).unwrap(),
            _ => println!("No webhook!")
        };
        println!("Message: {}", line);
    };

    loop {
        server.handle_recieved_lines(&action);
        thread::sleep(Duration::from_millis(50));
    }
}

fn read_config_file(file_name: &str) -> Config {
    let mut config_str = String::from("");
    let config_file = File::open(file_name);
    match config_file {
        Ok(mut file) => file.read_to_string(&mut config_str).unwrap(),
        _ => 0
    };
    toml::from_str(&config_str).expect("Invalid TOML")
}