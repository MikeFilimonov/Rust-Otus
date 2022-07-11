use std::{
    error::Error,
    fmt,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    // process,
    // str,
};

pub struct SmartOutletClient{
    stream: TcpStream
}

pub enum SmartOutletState{
    On,
    Off,
    Wasted
}

pub enum Command{
    CheckState,
    CheckConsumption,
    ToggleState,
    Explode
}

impl From<u8> for Command {
    fn from(val: u8) -> Self{
        match val {
            0 => Self::CheckConsumption,
            1 => Self::CheckState,
            2 => Self::ToggleState,
            _ => Self::Explode
        }
    }

}

impl From<Command> for u8 {

    fn from(command: Command) -> u8{

        match command {
            
            Command::CheckConsumption => 0,
            Command::CheckState => 1,
            Command::Explode => 255,
            Command::ToggleState => 2
        }

    }

}

pub enum SmartOutletServerResponse{

    SmartOutletState(u32),
    Wattage(f32),
    Report(String),
    TBD

}

impl From <[u8; 5]> for SmartOutletServerResponse{

    fn from(incoming_data: [u8; 5]) -> Self {

        match incoming_data {

            [0, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&incoming_data[1..]);
                Self::Wattage(f32::from_be_bytes(buf))
            },
            [1, ..] =>{
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&incoming_data[1..]);
                Self::Report(String::from_utf8(buf.to_vec()).unwrap())
             },
            [2, ..] =>{
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&incoming_data[1..]);
                Self::SmartOutletState(u32::from_be_bytes(buf))
            },
            _ => Self::TBD

        }

    }

}

impl From <SmartOutletServerResponse> for [u8; 5] {

    fn from(response: SmartOutletServerResponse) -> Self{

        let mut buffer = [0u8; 5];

        match response {

            SmartOutletServerResponse::SmartOutletState(val) => {
                buffer[0] = 2;
                buffer[1..].copy_from_slice(&val.to_be_bytes())
            },
            SmartOutletServerResponse::Wattage(val) => {
                buffer[0] = 0;
                buffer[1..].copy_from_slice(&val.to_be_bytes())
            },
            SmartOutletServerResponse::Report(val) => {
                buffer[0] = 1;
                buffer[1..].copy_from_slice(&val.as_bytes())
            },     
            TBD => buffer[0] = 255

        };
        
        buffer

    }

}

impl fmt::Display for SmartOutletServerResponse {

    fn fmt(&self, f: &mut  fmt::Formatter<'_>) -> fmt::Result{
        match self {
            
            SmartOutletServerResponse::SmartOutletState(val) => write!(f, "The smart outlet is on: {}", val),
            SmartOutletServerResponse::Report(val) => write!(f, "Summary on the smart outlet state: {}", val),            
            SmartOutletServerResponse::Wattage(val) => write!(f, "Current consumption is: {} W", val),
            SmartOutletServerResponse::TBD => write!(f, "Unexpected command. Execution gonna be terminated."),

        }
    }
}


impl SmartOutletClient {

    pub fn new(server: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>>{

        let stream = TcpStream::connect(server)?;
        Ok(Self{stream})

    }

    pub fn execute(&mut self, command: Command) -> Result <SmartOutletServerResponse, Box<dyn Error>>{

        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())

    }


}

