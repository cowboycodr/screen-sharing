mod server;
use server::{Server};

fn main() {
  let server = Server::new(
    (127, 0, 0, 1),
    8080
  );

  let ip = server.get_ip();
  let port = server.get_port();

  println!("Server is being hosted on: {:?}:{}", ip, port);

  server.start().unwrap();
}