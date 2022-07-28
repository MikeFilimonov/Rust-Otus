use async_udp_server::{consts, Thermometer};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let receiver_address = consts::DEFAULT_THERMO_ADDRESS;
    let thermometer = Thermometer::new(receiver_address).await.unwrap();

    for _ in 0..consts::MAX_ITERAION_VALUE {
        time::sleep(Duration::from_secs(1)).await;
        let value = thermometer.get_data().await;
        println!("Now it's {value} degrees C");
    }
}
