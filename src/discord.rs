use std::collections::HashMap;

pub struct DiscordWebhook {
    url: String,
    client: reqwest::blocking::Client,
}

impl DiscordWebhook {
    pub fn new(url: &String) -> DiscordWebhook {
        DiscordWebhook { 
            url: String::from(url),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn post_message(&self, item: &impl DiscordFormatable) -> Result<(), Box<dyn std::error::Error>> {
        let mut message = HashMap::new();
        message.insert(String::from("content"), item.get_content());

        self.client.post(&self.url)
            .json(&message)
            .send()?;

        Ok(())
    }

    pub fn post_string(&self, s: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut message = HashMap::new();
        message.insert(String::from("content"), s);

        self.client.post(&self.url)
            .json(&message)
            .send()?;

        Ok(())
    }
}

pub trait DiscordFormatable {
    fn get_content(&self) -> String;
}