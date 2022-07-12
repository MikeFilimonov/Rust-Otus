use std::{
    error::Error,
    fmt,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

pub mod consts {
    pub const DEFAULT_OUTLET_ADDRESS: &str = "127.0.0.1:7878";
}

pub struct SmartOutletClient {
    stream: TcpStream,
}

#[derive(Debug)]
pub enum Command {
    CheckState,
    CheckConsumption,
    ToggleState,
    Explode,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::CheckConsumption,
            1 => Self::CheckState,
            2 => Self::ToggleState,
            _ => Self::Explode,
        }
    }
}

impl From<Command> for u8 {
    fn from(command: Command) -> Self {
        match command {
            Command::CheckConsumption => 0,
            Command::CheckState => 1,
            Command::Explode => 255,
            Command::ToggleState => 2,
        }
    }
}

#[derive(Debug)]
pub enum ServerResponse {
    State(u32),
    Wattage(f32),
    Report(u32),
    TBD,
}


impl From<[u8; 5]> for ServerResponse {
    fn from(incoming_data: [u8; 5]) -> Self {
        match incoming_data {
            [0, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&incoming_data[1..]);
                Self::Wattage(f32::from_be_bytes(buf))
            },
            [1, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&incoming_data[1..]);
                Self::Report(u32::from_be_bytes(buf))

            },
            [2, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&incoming_data[1..]);
                Self::State(u32::from_be_bytes(buf))
            },
            _ => Self::TBD,
        }
    }
}

impl From<ServerResponse> for [u8; 5] {
    fn from(response: ServerResponse) -> Self {
        
        let mut buffer = [0u8; 5];

        match response {
            ServerResponse::State(val) => {
                buffer[0] = 2;
                buffer[1..].copy_from_slice(&val.to_be_bytes())
            }
            ServerResponse::Wattage(val) => {
                buffer[0] = 0;
                buffer[1..].copy_from_slice(&val.to_be_bytes())
            }
            ServerResponse::Report(val) => {
                buffer[0] = 1;
                buffer[1..].copy_from_slice(&val.to_be_bytes())
            }
            ServerResponse::TBD => buffer[0] = 255,
        };

        buffer
    }
}


impl fmt::Display for ServerResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            
            ServerResponse::State(val) => {
                let state =  match val{
                    0u32 => " turned off",
                    1u32 => " turned on",
                    _ => " broken",
                };
                write!(f, "The smart outlet is  {}", state)
            },
            ServerResponse::Report(val) => {
                let state =  match val{
                    0u32 => "off",
                    1u32 => "on",
                    _ => "broken",
                };
                write!(f, "The current outlet state is : {}", state)

            },
            ServerResponse::Wattage(val) => write!(f, "Current consumption is: {} W", val),
            ServerResponse::TBD => write!(f, "Unexpected command. Execution gonna be terminated.")

        }
    }
}

impl SmartOutletClient {
    pub fn new(server: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server)?;
        Ok(Self { stream })
    }

    pub fn execute(&mut self, command: Command) -> Result<ServerResponse, Box<dyn Error>> {

        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer)?;

        Ok(buffer.into())
    }
}
