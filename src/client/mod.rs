use std::net::{
  TcpStream,
  SocketAddrV4,
  Ipv4Addr
};

use std::net::tcp

pub struct Client {
  address: SocketAddrV4
}

impl Client {
  pub fn new<T>(address: (u8, u8, u8, u8), port: u16) -> Client {
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
    let mut stream = TcpStream::connect(self.address).unwrap();

    let _ = stream.write(&[1]);
  } 
}