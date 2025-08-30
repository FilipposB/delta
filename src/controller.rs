use log::{error, info};
use std::io::{ErrorKind, Read};
use std::net::{SocketAddr, TcpStream};

const TCP_BUFFER_SIZE: usize = 1024;

pub struct Controller {
    tcp_stream: TcpStream,
    addr: SocketAddr,
}

impl Controller {
    pub fn new(tcp_stream: TcpStream, addr: SocketAddr) -> Controller {
        Controller { tcp_stream, addr }
    }

    pub fn tick(&mut self) {
        let mut buffer = [0; TCP_BUFFER_SIZE];
        match self.tcp_stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return;
                }
                info!(
                    "TCP client {} sent {}",
                    self.addr,
                    String::from_utf8_lossy(&buffer[0..bytes_read])
                );
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                ErrorKind::ConnectionReset => {}
                _ => {
                    error!("{} dd", err);
                }
            },
        }
    }
}
