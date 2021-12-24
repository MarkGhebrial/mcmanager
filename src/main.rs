use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() -> Result<(), Error> {
    //let command = Command::new("python3").arg("testproc.py");
    let command = Command::new("java").arg("-Xmx1024M").arg("-Xms1024M").arg("-jar").arg("server.jar").arg("nogui")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = command.stdin.unwrap();
    let stdout = command.stdout.unwrap();

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("Recieved: {}", line));

    Ok(())
}
