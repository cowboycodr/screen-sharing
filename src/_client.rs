mod client;
use client::{Client};

fn main() {
  let client = Client::new(
    (127, 0, 0, 1),
    8080
  );

  client.connect();
}