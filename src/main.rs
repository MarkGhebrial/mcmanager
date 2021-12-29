use std::fs::File;
use std::io::prelude::*;
use std::{ thread, time::Duration };

use serde_derive::Deserialize;

mod server_manager;
mod discord;
use server_manager::*;
use discord::*;

#[derive(Deserialize)]
struct Config {
    discord_url: Option<String>,
    server_directory: Option<String>
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
    
    //{ action(String::from("testing")); }

    let mut server = Manager::new();
    if let Some(s) = config.server_directory {
        server.server_directory(&s);
    }
    server.start();

    let action = |line: String| {
        /*match &webhook {
            Some(w) => w.post_string(&line).unwrap(),
            _ => println!("No webhook!")
        };*/
        println!("Message: {}", line);
        if let Some(_) = line.find("[Server thread/INFO]: Done") {
            thread::sleep(Duration::from_secs(5));
            println!("Stopping the server");
            server.send_command(String::from("stop")).unwrap();
        }
    };

    loop {
        server.handle_recieved_lines(&action);
    }
}
