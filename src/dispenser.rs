use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use log::{debug, error};
use crate::dispensable_data::DispensableData;

struct  ConnectionState {
    id: String,
    chunk_index: u64,
    socket_addr: SocketAddr,
}

impl ConnectionState {
    
    fn new(id: &str, socket_addr: SocketAddr) -> ConnectionState {
        ConnectionState {
            id: id.parse().unwrap(),
            chunk_index: 0,
            socket_addr
        }
    }

}

pub struct Dispenser {
    connections: Vec<ConnectionState>,
    dispensable_data: DispensableData
}

impl Dispenser {
    pub fn new(dispensable_data: DispensableData) -> Dispenser {
        Dispenser {
            connections: Vec::new(),
            dispensable_data
        }
    }

    pub fn add_connection(&mut self, addr: SocketAddr) {
        self.connections.push(ConnectionState::new("", addr));
    }
    
    pub fn tick(&mut self, udp_socket: &UdpSocket) {
        

        let mut sorted_connections: HashMap<u64, Vec<&mut ConnectionState>> = HashMap::new();

        self.connections.iter_mut().for_each(|connection| {
            if connection.chunk_index < self.dispensable_data.get_total_chunks() {
                sorted_connections.entry(connection.chunk_index).or_insert(Vec::new()).push(connection);
            }
        });

        sorted_connections.iter_mut().for_each(|(chunk, connections)| {
            let chunk_data = self.dispensable_data.load_chunk(*chunk, true, 0);

            match chunk_data {
                Ok(chunk_data) => {
                    //Add chunk number before the data
                    
                    let mut package = chunk.to_be_bytes().to_vec();
                    package.extend_from_slice(&chunk_data);
                    
                    connections.iter_mut().for_each(|connection| {
                        match udp_socket.send_to(&*package, connection.socket_addr) {
                            Ok(_) => {
                                connection.chunk_index += 1;
                            }
                            Err(err) => {
                                error!("Error sending chunk: {:?}", err);
                            }
                        }
                    })
                }
                Err(e) => {error!("Error loading chunk: {}", e)}
            }


        })

    }
}