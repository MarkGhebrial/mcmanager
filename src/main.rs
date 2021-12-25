use std::fs::File;
use std::io::prelude::*;

use serde_derive::Deserialize;

mod server_manager;
mod discord;
use server_manager::*;
use discord::*;

#[derive(Deserialize)]
struct Config {
    discord_url: Option<String>
}

fn main() {
    let mut config_str = String::from("");
    let config_file = File::open("mcmanager.toml");
    match config_file {
        Ok(mut file) => file.read_to_string(&mut config_str).unwrap(),
        _ => 0
    };
    let config: Config = toml::from_str(&config_str).expect("Invalid TOML");

    let webhook: Option<DiscordWebhook> = match config.discord_url {
        Some(s) => Some(DiscordWebhook::new(&s)),
        None => None
    }; 

    let action = move |line| {
        match &webhook {
            Some(w) => w.post_string(&line).unwrap(),
            _ => println!("No webhook!")
        };
        println!("Message: {}", line);
    };
    //{ action(String::from("testing")); }

    let mut server = Manager::new();
    server.start();
    loop {
        server.handle_recieved_lines(&action);
    }
}
