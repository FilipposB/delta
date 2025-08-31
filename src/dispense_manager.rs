use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use log::error;
use crate::dispensable_data::DispensableData;
use crate::dispenser::Dispenser;

pub struct DispenseManager {
    dispensers: HashMap<String, Dispenser>,
}

impl DispenseManager{
    pub fn new() -> DispenseManager{

        let mut dispensers: HashMap<String, Dispenser> = HashMap::new();
        match DispensableData::new("sample", 1012, 1973) {
            Ok(data) => {
                dispensers.insert("dd".parse().unwrap(), Dispenser::new(data));
            },
            Err(err) => {
                error!("{}", err);
            },
        };
        
        DispenseManager{
            dispensers,
        }
    }

    pub fn tick(&mut self, udp_socket: &UdpSocket) {
        self.dispensers.iter_mut().for_each(|(_, dispenser)| {dispenser.tick(udp_socket)})
    }

    pub fn add_to_dispenser(&mut self, dispenser_id: &str, addr: &SocketAddr) {
        match self.dispensers.get_mut(dispenser_id) {
            Some(dispenser) => {
                dispenser.add_connection(*addr);
            }
            _ => {}
        }
    }
}