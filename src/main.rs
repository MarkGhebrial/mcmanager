mod server_manager;
use server_manager::*;

fn main() {
    let mut server = Manager::new();
    server.start();
    loop {
        server.handle_recieved_lines(|line| {
            println!("{}", line);
        });
    }
}
