mod server;
use server::{ Server };

mod client;
use client::{ Client };

fn main() {
    let server = Server::new(
        (127, 0, 0, 1),
        3000
    );

    let client = Client::new(server);
}