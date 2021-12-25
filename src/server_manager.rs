//! Takes care of communication with the server instance

use std::thread;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::sync::mpsc;

pub struct Manager {
    /// The shell command to start the Minecraft server
    start_command: String,

    /// `mpsc` reciever to capture stdout from the server process
    reciever: Option<mpsc::Receiver<String>>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            start_command: String::from("java -Xmx1024M -Xms1024M -jar server.jar nogui"),
            reciever: None
        }
    }

    /// Overrides the default shell command to start the server, which is
    /// `java -Xmx1024M -Xms1024M -jar server.jar nogui`
    /// 
    /// # Example
    /// ```rust
    /// let server = server_manager::Manager::new()
    ///     .shell_command("java -Xmx1024M -Xms1024M -jar server.jar");
    /// ```
    pub fn shell_command(mut self, s: &str) -> Manager {
        self.start_command = String::from(s);
        self
    }

    /// Start the server in a new thread that listens to its stdout stream
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel(); // Create a channel between the threads
        self.reciever = Some(rx);

        // Parse the arguments in the start command
        let args: Vec<&str> = self.start_command.rsplit(' ').rev().collect();

        // Ownership of `command` is moved to the new thread
        let command = Command::new(args[0]).args(&args[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().unwrap();

        thread::spawn(move || {
            let stdin = command.stdin.unwrap();
            let stdout = command.stdout.unwrap();

            let reader = BufReader::new(stdout);

            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| {
                    tx.send(line).unwrap()
                });
        });
    }

    /// Call the closure for each line in the reciever's buffer
    pub fn handle_recieved_lines<F>(&self, f: &F) where F: Fn(String) {
        if let Some (rx) = &self.reciever {
            for line in rx.try_iter() {
                println!("Executing closure");
                f(line);
            }
        }
    }
}