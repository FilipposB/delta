mod request;
mod manifest;

use std::cmp::min;
use std::collections::VecDeque;
use log::{debug, error, info};
use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use crate::controller::ControllerAction::{PROCESS, RESET};
use crate::controller::request::{Request, RequestData, RequestFactory, RequestState};

const TCP_BUFFER_SIZE: usize = 1024;

enum ControllerAction {
    RESET(String),
    PROCESS
}

pub struct Controller {
    tcp_stream: TcpStream,
    addr: SocketAddr,
    buffer: [u8; TCP_BUFFER_SIZE],
    cached_buffer: VecDeque<u8>,
    active_request: Option<Request>,
}

impl Controller {
    pub fn new(tcp_stream: TcpStream, addr: SocketAddr) -> Controller {
        Controller {
            tcp_stream,
            addr ,
            buffer: [0; TCP_BUFFER_SIZE],
            cached_buffer: VecDeque::new(),
            active_request: None
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

        let controller_action = match self.active_request {
            Some(_) => {
                PROCESS
            }
            None => {
                let new_request = RequestFactory::get_request(self.cached_buffer.pop_front().unwrap());
                match new_request {
                    Some(request) => {
                        self.active_request = Some(request);
                        PROCESS
                    }
                    None => {
                        RESET("Something went wrong will reset the connection".parse().unwrap())
                    }
                }
            }
        };

        match controller_action {
            PROCESS => {
                if let Some(request) = self.active_request.as_mut() {
                    
                    match request.get_length() {
                        Some(_) => {}
                        None => {
                            if self.cached_buffer.len() < 8 {
                                return;
                            }
                            let bytes: [u8; 8] = self.cached_buffer
                                .drain(0..8)
                                .collect::<Vec<u8>>()
                                .try_into()
                                .expect("drain must return 8 bytes");

                            request.init_length(u64::from_be_bytes(bytes));                        }
                    }
                    
                    let pending_bytes = request.pending_bytes();
                    let bytes_to_be_used = min(self.cached_buffer.len(), pending_bytes);
                    let drained: Vec<u8> = self.cached_buffer.drain(0..bytes_to_be_used).collect();
                    match request.add(&drained) {
                        RequestState::PENDING => {}
                        RequestState::COMPLETED(request_data) => {
                            
                            match request_data {
                                RequestData::DownloadResource(manifest) => {
                                    self.tcp_stream.write(manifest.id.as_bytes()).unwrap();
                                }
                            }
                            
                            info!("Request completed successfully {:?}",request);
                            self.active_request = None;
                        },
                        RequestState::ERROR(error) => error!("Request completed with error {:?}", error),
                    }
                }
            }
            RESET(reason) => {
                error!("{} reset the connection", reason);
                let _ = self.tcp_stream.shutdown(Shutdown::Both);
                self.active_request = None;
                self.cached_buffer.clear();
            }
        }

    }
    
}
