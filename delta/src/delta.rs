use std::io::ErrorKind;
use std::net::{TcpListener, UdpSocket};
use log::{debug, error, info};
use crate::controller::Controller;
use crate::dispense_manager::DispenseManager;

const MAX_NEW_CONNECTIONS_PER_TICK: usize = 10;
const UDP_BUFFER_SIZE: usize = 1024;

pub struct Delta {
    tcp_listener: TcpListener,
    controllers: Vec<Controller>,
    udp_socket: UdpSocket,
    dispense_manager: DispenseManager,
}

impl Delta {

    pub fn new(port: u16)  -> Delta {
        let tcp_listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

        tcp_listener.set_nonblocking(true).unwrap();

        let udp_socket = UdpSocket::bind(("127.0.0.1", port)).unwrap();

        udp_socket.set_nonblocking(true).unwrap();

        Delta {
            tcp_listener,
            controllers: Vec::new(),
            udp_socket,
            dispense_manager: DispenseManager::new(),
        }

    }

    fn listen_for_new_controller_clients(&mut self){
        for _ in 0..MAX_NEW_CONNECTIONS_PER_TICK {
            match self.tcp_listener.accept() {
                Ok((stream, addr)) => {
                    stream.set_nonblocking(true).unwrap();
                    info!("New connection from {}", addr);
                    self.controllers.push(Controller::new(stream, addr));
                }
                Err(err) => {
                    match err.kind() {
                        ErrorKind::WouldBlock => {
                        }
                        ErrorKind::ConnectionReset => {
                            error!("{}", err);
                        }
                        _ => {}
                    }

                }
            }
        }
    }

    fn listen_for_new_udp_clients(&mut self) {
        let mut buffer: [u8; UDP_BUFFER_SIZE] = [0; UDP_BUFFER_SIZE];
        for _ in 0..MAX_NEW_CONNECTIONS_PER_TICK {
            match self.udp_socket.recv_from(&mut buffer){
                Ok(x) => {
                    let id = String::from_utf8_lossy(&buffer[0..x.0]);
                    self.dispense_manager.add_to_dispenser(&*id, &x.1);
                    info!("UDP client from {} sent {}", x.1, id);
                }
                Err(err) => {
                    match err.kind() {
                        ErrorKind::WouldBlock => {
                        }
                        ErrorKind::ConnectionReset => {
                            
                            break;
                        }
                        _ => {
                            error!("{} dd", err);
                        }
                    }
                }
            }
        }
    }

    pub fn tick(&mut self){
        self.listen_for_new_controller_clients();
        self.controllers.iter_mut().for_each(|controller| controller.tick());
        self.listen_for_new_udp_clients();
        self.dispense_manager.tick(&self.udp_socket);
    }


}

