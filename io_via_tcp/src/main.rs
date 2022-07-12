use io_via_tcp::{consts, Command, ServerResponse};
use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
};
use rand::{self, Rng};


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
        
        println!("toggle state {:?}", command);

        match command {
            Command::ToggleState => {
                
               println!("state now: {:?}", self.state);

                let new_state = match self.state {
                   
                    0u32 => 1u32,
                    1u32 => 0u32,
                    _ => 7u32,
                };
                self.state = new_state;
                self.wattage = match new_state {
                    
                    1u32 => {
                        let mut range = rand::thread_rng();
                        range.gen_range(220.0 .. 229.9) as f32 },
                    0u32 => {
                        let mut range = rand::thread_rng();
                        range.gen_range(0.0 .. 0.9) as f32 },
                    _ => -1.0f32,

                };

                println!("wattage {:?}", self.wattage);

                println!("new state {:?}", new_state);
                ServerResponse::State(new_state)               
            }
            Command::CheckConsumption => ServerResponse::Wattage(self.wattage),
            Command::CheckState => {
                // let state = match self.state {
                //     1u32 => "on",
                //     0u32 => "off",
                //     _ => "broken",
                // };

                // let report = format!(" The device is {}, consumes {} W", state, self.wattage);
                // println!("{:?}", report);

                ServerResponse::Report(self.state)

            }
            Command::Explode => ServerResponse::TBD,
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let server_address = args
        .next()
        .unwrap_or_else(|| consts::DEFAULT_OUTLET_ADDRESS.into());

    let port_monitor =
        TcpListener::bind(server_address).expect("Failed to bind to the given address");

    let mut smart_outlet_emulator = SmartOutletEmulator::new();

    while let Some(connection) = port_monitor.incoming().next() {
        let mut stream = match connection {
            Ok(result) => result,
            Err(error) => {
                println!("Can't establish connection due to error:{}", error);
                continue;
            }
        };

        let client = stream
            .peer_addr()
            .map(|address| address.to_string())
            .unwrap_or_else(|_| "failed to get client ip".into());

        println!("New client has connected, ip {}", client);

        let mut data = [0u8];

        while stream.read_exact(&mut data).is_ok() {
            let response = smart_outlet_emulator.interact(data[0].into());
            let data: [u8; 5] = response.into();
            if stream.write_all(&data).is_err() {
                break;
            };
        }

        println!(
            "Client {} has been disconnected. Ready for a new connection",
            client
        );
    }
}
