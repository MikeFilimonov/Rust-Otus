use async_tcp_server::{consts, Command, ServerResponse};
use rand::{self, Rng};
use std::{
    env,
    sync::Arc,
    // io::{Read, Write},
    // net::TcpListener,
};
pub use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

struct SmartOutletEmulator {
    state: u32,
    wattage: f32,
}

impl SmartOutletEmulator {
    fn new() -> Self {
        Self {
            state: 0u32,
            wattage: 0f32,
        }
    }

    fn interact(&mut self, command: Command) -> ServerResponse {
        match command {
            Command::ToggleState => {
                let new_state = match self.state {
                    0u32 => 1u32,
                    1u32 => 0u32,
                    _ => 7u32,
                };
                self.state = new_state;
                self.wattage = match new_state {
                    1u32 => {
                        let mut range = rand::thread_rng();
                        range.gen_range(220.0..229.9) as f32
                    }
                    0u32 => {
                        let mut range = rand::thread_rng();
                        range.gen_range(0.0..0.9) as f32
                    }
                    _ => -1.0f32,
                };

                ServerResponse::State(new_state)
            }
            Command::CheckConsumption => ServerResponse::Wattage(self.wattage),
            Command::CheckState => ServerResponse::Report(self.state),
            Command::Explode => ServerResponse::TBD,
        }
    }
}

#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let server_address = args
        .next()
        .unwrap_or_else(|| consts::DEFAULT_OUTLET_ADDRESS.into());

    let port_monitor = TcpListener::bind(server_address)
        .await
        .expect("Failed to bind to the given address");

    let smart_outlet_emulator = Arc::new(Mutex::new(SmartOutletEmulator::new()));

    while let Ok((mut stream, address)) = port_monitor.accept().await {
        println!("New client has conneced, ip {}", address);

        let smart_outlet_emulator = smart_outlet_emulator.clone();
        tokio::spawn(async move {
            let mut data = [0u8];
            while stream.read_exact(&mut data).await.is_ok() {
                let response = smart_outlet_emulator.lock().await.interact(data[0].into());

                let data: [u8; 5] = response.into();
                if stream.write_all(&data).await.is_err() {
                    break;
                };
            }

            println!(
                "Client {} has been disconnected. Ready for a new connection",
                address
            );
        });
    }
}
