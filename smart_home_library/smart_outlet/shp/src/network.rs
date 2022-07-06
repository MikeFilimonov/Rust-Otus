use std::io::{Read, Write};
use std::mem::size_of;
use std::net::SocketAddr;

use crate::wrappers::SHTCPError;

pub mod constants {

    pub const DEFAULT_TCP_SERVER_ADDR: &str = "127.0.0.1:7878";
    pub const DEFAULT_TCP_CLIENT_ADDR: &str = "127.0.0.1:1489";
}

pub enum Command{

    ChangeState,
    GetState,
    GetConsumption,
    Undefined
}

impl From<u8> for Command{

    fn from(val:u8) -> Self {

        match val {

            0 => Self::ChangeState,
            1 => Self::GetState,
            2 => Self::GetConsumption,
            _ => Self::Undefined

        }

    }

}

impl From<Command> for Command {

    fn from(val: Command){-> u8{

        match val {

            ChangeState => 0,
            GetState => 1,
            GetConsumption => 2,
            Undefined => 255,

        }

    }
        
    }
}


pub enum SocketServerResponse {

    Done,
    CurrentState(u8),
    CurrentConsumption(f32),
    Undefined

}
