//https://en.wikipedia.org/wiki/Type%E2%80%93length%E2%80%93value

use std::io::{Read, Write};
use bincode::{self, Options};
use serde::{de, Deserialize, Serialize};

pub mod wrappers;
pub mod network;
// pub use wrappers::*;
use wrappers::*;

pub trait Message{
    const MESSAGE_TYPE:u16;
}


pub (crate) fn send_message <M: Message + Serialize, W:Write>(message: M, mut writer: W) 
    -> Result<(), SHTCPError>{

    let buffer = M::MESSAGE_TYPE.to_be_bytes();
     writer.write_all(&buffer)?;

    let serialized_data = bincode::options().with_big_endian().serialize(&message)?;
    let size = serialized_data.len() as u16;
    let bytes = size.to_be_bytes();
    writer.write_all(&bytes)?;
    writer.write_all(serialized_data.as_ref())?;

    Ok(())

}

pub (crate) fn receive_message<Response: Message + de::DeserializeOwned, R:Read>(mut reader: R) 
    -> Result< Box<Response>, SHTCPError> {

        let mut buffer = [0u8, 2];
        reader.read_exact(&mut buffer)?;

        let message_type = u16::from_be_bytes(buffer);

        if message_type != Response::MESSAGE_TYPE {
            return Err(SHTCPError::WrongMessageType(message_type));
        };

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let size = u32::from_be_bytes(buffer);

        // let mut buffer = [0u8, size as _];
        let mut data = vec![0u8, size as _];
        reader.read_exact(&mut data);

        // let message = bincode::deserialize_from(reader)?;
        let message = bincode::options().
            with_big_endian().
            deserialize(&data[..])?;


        Ok(Box::new(message))

}