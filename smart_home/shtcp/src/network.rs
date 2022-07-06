use std::io::{Read, Write};
use std::mem::size_of;
use std::net::SocketAddr;

use crate::wrappers::SHTCPError;

type SizeType = u64;

pub mod constants {

    pub const DEFAULT_TCP_ADDR: &str = "127.0.0.1:7878";
    
    pub const ENABLE_SMART_OUTLET_COMMAND: &str = "SO_ON";
    pub const DISABLE_SMART_OUTLET_COMMAND: &str = "SO_OFF";
    pub const GET_SMART_OUTLET_STATE_COMMAND: &str = "SO_STATE";

}

// pub fn send_command -> Result<(), SHTCPError> {

// }