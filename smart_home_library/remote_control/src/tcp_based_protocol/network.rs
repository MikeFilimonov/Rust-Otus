use std::io::{Read, Write};
use std::mem::size_of;
use std::net::SocketAddr;

use crate::tcp_based_protocol::wrappers::SHTCPError;

pub mod consts {

    pub const DEFAULT_TCP_SERVER_ADDR: &str = "127.0.0.1:7878";
    pub const DEFAULT_TCP_CLIENT_ADDR: &str = "127.0.0.1:1489";

    pub const RQST_ID: u16 = 0x1;
    pub const RSPNS_ID: u16 = 0xF;
    pub const TEXT_MSG_ID: u16 = 0xABCD;
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

// impl From<Command> for Command {

//     fn from(val: Command){-> u8{

//         match val {

//             ChangeState => 0,
//             GetState => 1,
//             GetConsumption => 2,
//             Undefined => 255,

//         }

//     }
        
//     }
// }


pub enum SocketServerResponse {

    Done,
    CurrentState(u8),
    CurrentConsumption(f32),
    Undefined

}

