extern crate core;

use crate::delta::Delta;
use crate::dispensable_data::DispensableData;
use crate::dispenser::Dispenser;

mod dispenser;
mod dispensable_data;
mod dispense_manager;
mod delta;
mod controller;

fn main() {

    env_logger::init();
    let mut delta = Delta::new(3332);

    loop {
        delta.tick()
    }
}
