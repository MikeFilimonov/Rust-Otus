use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

pub mod consts {

    pub const DEFAULT_THERMO_ADDRESS: &str = "127.0.0.1:7878";
    pub const DEFAULT_UDP_CLIENT: &str = "127.0.0.1:2023";
    pub const MAX_ITERAION_VALUE: u32 = 100;
}

pub struct DataEmulator {
    session_start: Instant,
}

#[derive(Default)]
pub struct SharedValue(Mutex<f32>);

pub struct Thermometer {
    pub is_ready: Arc<AtomicBool>,
    temperature: Arc<SharedValue>,
}

impl DataEmulator {
    pub fn emulate(&self) -> f32 {
        let pause = Instant::now() - self.session_start;
        10.0 + (pause.as_secs_f32() / 2.0).cos()
    }
}

impl Default for DataEmulator {
    fn default() -> Self {
        Self {
            session_start: Instant::now(),
        }
    }
}

impl SharedValue {
    pub fn get_value(&self) -> f32 {
        *self.0.lock().unwrap()
    }

    pub fn set_value(&self, value: f32) {
        *self.0.lock().unwrap() = value
    }
}

impl Thermometer {
    pub fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let connection = UdpSocket::bind(address)?;
        let timeout = Some(Duration::from_secs(1));
        connection.set_read_timeout(timeout)?;

        let is_ready = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(SharedValue::default());

        let shared_availability = is_ready.clone();
        let shared_result = temperature.clone();

        thread::spawn(move || loop {
            if shared_availability.load(Ordering::SeqCst) {
                return;
            }

            let mut buffer = [0; 4];

            if let Err(err) = connection.recv_from(&mut buffer) {
                println!("Can't retrieve data from thermometer because of {err}");
            }

            let received_value = f32::from_be_bytes(buffer);
            shared_result.set_value(received_value);
        });

        Ok(Self {
            is_ready,
            temperature,
        })
    }

    pub fn get_data(&self) -> f32 {
        self.temperature.get_value()
    }
}
