use std::io::prelude::*;
use std::net::{ SocketAddrV4, Ipv4Addr, TcpListener };
use std::io::{ Error };

/// Simple TCP server
pub struct Server {

  /// The IP the server will be hosted on
  ip: (u8, u8, u8, u8),
  /// The port the server will be hosted on
  port: u16,
}

impl Server {
  /// Returns a new instance of the Server struct
  pub fn new(ip: (u8, u8, u8, u8), port: u16) -> Server {
    Server {
      ip: ip,
      port: port
    }
  }

  /// Starts listening for incoming connections and messages
  pub fn start(&self) -> Result<(), Error> {
    let loopback = Ipv4Addr::new(
      self.ip.0,
      self.ip.1,
      self.ip.2,
      self.ip.3,
    );

    let socket = SocketAddrV4::new(loopback, self.port);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;

    println!("Server started on http://{}", port);

    for stream in listener.incoming() {
      match stream {
        Ok(mut stream) => {
          println!("Connection established: {}", stream.peer_addr().unwrap());

          let mut buffer = [0; 1024];
          stream.read(&mut buffer).unwrap();
        }
        Err(e) => {
          println!("Error: {}", e);
          // Connection failed
        }
      }
    }

    drop(listener);
    Ok(())
  }

  /// Gets the ip the server is being hosted on
  pub fn get_ip(&self) -> (u8, u8, u8, u8) {
    return self.ip;
  }

  /// Gets the port the server is being hosted on
  pub fn get_port(&self) -> u16 {
    return self.port;
  }
}