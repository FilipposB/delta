use crate::dispensable_data::DispensableData;
use crate::dispenser::Dispenser;
use log::error;
use std::collections::{HashMap, HashSet};
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use uuid::Uuid;
use delta_lib::object::manifest::Manifest;

pub struct DispenseManager {
    dispensers: HashMap<Uuid, Dispenser>,
    subscribers: HashMap<Uuid, Uuid>,
}

impl DispenseManager {
    pub fn new() -> DispenseManager {
        let mut dispensers: HashMap<Uuid, Dispenser> = HashMap::new();
        match DispensableData::new("IriunWebcam-2.8.10.exe", "IriunWebcam-2.8.10.exe", 35535, 3005) {
            Ok(data) => {
                dispensers.insert(
                    Uuid::from_str("d7bf0ada-7e54-4775-9309-ac2d41f6ee80").expect("REASON"),
                    Dispenser::new(data),
                );
            }
            Err(err) => {
                error!("{}", err);
            }
        };

        DispenseManager { dispensers, subscribers: HashMap::new() }
    }

    pub fn tick(&mut self, udp_socket: &UdpSocket) {
        self.dispensers
            .iter_mut()
            .for_each(|(_, dispenser)| dispenser.tick(udp_socket))
    }

    pub fn add_to_dispenser(&mut self, subscriber_id: Uuid, addr: &SocketAddr) {

        if let Some(dispenser_id) = self.subscribers.get_mut(&subscriber_id) {
            if let Some(dispenser) = self.dispensers.get_mut(dispenser_id) {
                dispenser.add_connection(subscriber_id, *addr);
            }
        }
        
    }

    pub fn manifest_for_user(&mut self, dispenser_id: &Uuid) -> Option<Manifest>  {
       match self.dispensers.get(dispenser_id) {
           None => {None}
           Some(dispenser) => {
               let subscriber_id = Uuid::new_v4();
               self.subscribers.insert(subscriber_id, dispenser_id.clone());
               let dispensable_data = dispenser.dispensable_data();
               Some(Manifest::new(subscriber_id, dispensable_data.name().parse().unwrap(), dispensable_data.chunk_size() as u64, dispensable_data.total_chunks(), dispensable_data.total_bytes()))
           }
       }
    }
}
