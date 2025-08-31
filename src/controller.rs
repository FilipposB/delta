use log::{debug, error};
use std::io::{ErrorKind, Read};
use std::net::{SocketAddr, TcpStream};

const TCP_BUFFER_SIZE: usize = 1024;

pub struct Controller {
    tcp_stream: TcpStream,
    addr: SocketAddr,
    buffer: [u8; TCP_BUFFER_SIZE],
    cached_buffer: Vec<u8>,
}

impl Controller {
    pub fn new(tcp_stream: TcpStream, addr: SocketAddr) -> Controller {
        Controller { 
            tcp_stream,
            addr ,
            buffer: [0; TCP_BUFFER_SIZE],
            cached_buffer: Vec::new()
        }
    }

    pub fn tick(&mut self) {
        match self.tcp_stream.read(&mut self.buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return;
                }
                debug!(
                    "TCP client {} sent {} Bytes",
                    self.addr,
                    bytes_read
                );
                self.cached_buffer.extend(self.buffer[0..bytes_read].iter());
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                ErrorKind::ConnectionReset => {}
                _ => {
                    error!("{} dd", err);
                }
            },
        }
        
        if self.cached_buffer.len() == 0 {
            return;
        }
        
        
        
    }
}
