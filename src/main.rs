extern crate core;

use crate::delta::Delta;
mod dispenser;
mod dispensable_data;
mod dispense_manager;
mod delta;

fn main() {

    env_logger::init();
    let mut delta = Delta::new(3332);

    loop {
        delta.tick()
    }
}
