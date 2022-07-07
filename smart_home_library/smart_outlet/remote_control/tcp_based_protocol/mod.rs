use std::io::{Read, Write};
use bincode::{self, Options};
use serde::{de, Deserialize, Serialize};

use crate::wrappers::*;

pub mod client;
pub mod server;
pub mod wrappers;
pub mod network;

pub trait Message{
    const MESSAGE_ID:u16;
}


pub (crate) fn send_message <M: Message + Serialize, W:Write>(message: M, &mut writer: W) -> Result<(), SHTCPError>{

    let bytes = M::type.to_be_bytes();
    writer.write_all(&bytes)?;

    let serialized_data = bincode::options().with_big_endian().serialize(&message)?;
    let size = serialized_data.len() as u16;
    let bytes_quantity = size.to_be_bytes();
    writer.write_all(&bytes_quantity)?;
    writer.write_all(serialized_data.as_ref())?;

    Ok(())

}