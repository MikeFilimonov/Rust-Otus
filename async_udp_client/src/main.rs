use async_udp_server::{consts, Thermometer};
use std::{thread, time::Duration};

fn main() {
    let receiver_address = consts::DEFAULT_THERMO_ADDRESS;
    let thermometer = Thermometer::new(receiver_address).unwrap();

    for _ in 0..consts::MAX_ITERAION_VALUE {
        thread::sleep(Duration::from_secs(1));
        let value = thermometer.get_data();
        println!("Now it's {value} degrees C");
    }
}
