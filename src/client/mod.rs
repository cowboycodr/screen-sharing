use std::io::*;
use std::str::from_utf8;
use std::net::{
  TcpStream,
  SocketAddrV4,
  Ipv4Addr
};

pub struct Client {
  address: SocketAddrV4
}

impl Client {
  pub fn new(address: (u8, u8, u8, u8), port: u16) -> Client {
    let ip_adress = Ipv4Addr::new(
        address.0,
        address.1,
        address.2,
        address.3
    );

    Client {
      address: SocketAddrV4::new(ip_adress, port),
    }
  }

  pub fn connect(&self) {
    match TcpStream::connect(self.address) {
      Ok(mut stream) => {
        println!("Connection established!");

        let msg = b"Hello!";

        stream.write_all(msg).unwrap();
        println!("Awaiting reply!");

        let mut data = [0 as u8; 6];
        match stream.read_exact(&mut data) {
          Ok(_) => {
            if &data == msg {
              println!("Reply is ok!");
            } else {
              let text = from_utf8(&data).unwrap();
              println!("Response: {}", text);
            }
          },
          Err(e) => {
            println!("Failed to recieve data: {}", e);
          }
        }
      },
      Err(e) => {
        println!("Failed to establish connection: {}", e);
      }
    }
  } 
}