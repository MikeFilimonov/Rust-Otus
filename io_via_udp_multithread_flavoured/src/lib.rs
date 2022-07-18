use std:: {
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex},
    thread,
    time::Duration,
};

pub struct Thermometer{
    is_ready: Arc<AtomicBool>,
    temperature: Arc<SharedValue>
}

#[derive(Default)]
pub struct SharedValue(Mutex<f32>);


impl Thermometer {

    pub fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>>{

        let connection = UdpSocket::bind(address)?;
        let timeout = Some(Duration::from_millis(500));       
        connection.set_read_timeout(timeout);

        let is_ready = Arc::new(AtomicBool::new(bool::default()));
        let temperature = Arc::new(SharedValue::default());

        Ok(Self{is_ready, temperature})

    }

}