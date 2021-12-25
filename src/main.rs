mod server_manager;
use server_manager::*;

fn main() {
    let mut server = Manager::new();
    server.start();
    loop {
        server.print_recieved_lines()
    }
}
