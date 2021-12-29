//! Takes care of communication with the server instance

use std::thread;
use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::mpsc;

pub struct Manager {
    /// The shell command to start the Minecraft server
    start_command: String,
    /// The directory containing the server's jar file
    working_dir: Option<String>,

    /// `mpsc` reciever to capture stdout from the server process
    reciever: Option<mpsc::Receiver<String>>,

    /// `mpsc` transmitter to send stdin to the server process
    transmitter: Option<mpsc::Sender<String>>,

    /// Holds the handles for the manager's helper threads, in  order
    /// to stop them when the server halts
    thread_handles: Vec<thread::JoinHandle<()>>,

    server_process: Option<Child>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            start_command: String::from("java -Xmx1024M -Xms1024M -jar server.jar nogui"),
            working_dir: None,
            reciever: None,
            transmitter: None,
            thread_handles: vec![],
            server_process: None
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
    pub fn shell_command(&mut self, s: &str) -> &mut Manager {
        self.start_command = String::from(s);
        self
    }

    pub fn server_directory(&mut self, s: &str) -> &mut Manager {
        self.working_dir = Some(String::from(s));
        self
    }

    /// Start the server in a new thread that listens to its stdout stream
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel(); // Create the channel the recieves stdout
        let (tx2, rx2) = mpsc::channel(); // Create the channel that sends stdin
        self.reciever = Some(rx);
        self.transmitter = Some(tx2);

        // Parse the arguments in the start command
        let args: Vec<&str> = self.start_command.rsplit(' ').rev().collect();

        // Ownership of `command` is moved to the new thread
        let mut command_builder = Command::new(args[0]);
        command_builder.args(&args[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());

        // Set the working directory for the process
        if let Some(s) = &self.working_dir {
            command_builder.current_dir(s);
        }

        // Start the process
        let command = command_builder.spawn().unwrap();
        
        //self.server_process = Some(command);

        let stdin = command.stdin.unwrap();
        let stdout = command.stdout.unwrap();

        let mut writer = BufWriter::new(stdin);
        let reader = BufReader::new(stdout);

        self.thread_handles.push(thread::spawn(move || {
            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| {
                    tx.send(line).unwrap()
                });
        }));
        self.thread_handles.push(thread::spawn(move || {
            loop {
                let msg = rx2.recv().unwrap();
                writer.write_all(&msg.into_bytes()).unwrap();
                writer.flush().unwrap();
            }
        }));
    }

    /// Stop the Minecraft server
    pub fn stop(&self) {
        self.send_command(String::from("stop")).unwrap(); // Gracefully stop the server
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(c) = &mut self.server_process {
            match c.try_wait().unwrap() {
                Some(_) => true,
                None => false
            }
        } else {
            false
        }
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

    /// Send a Minecraft command to the server's stdin
    /// 
    /// # Example
    /// ```rust
    /// let server = server_manager::Manager::new();
    /// // After the server has booted up
    /// server.send_command(String::from("say Hello from the server admin"))
    /// ```
    pub fn send_command(&self, s: String) -> Result<(), mpsc::SendError<String>> {
        if let Some(tx) = &self.transmitter {
            tx.send(s + "\n")?;
        }
        Ok(())
    }
}