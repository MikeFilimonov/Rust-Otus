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

    let bytes = M::MESSAGE_TYPE.to_be_bytes();
    writer.write_all(&bytes)?;

    let serialized_data = bincode::options().with_big_endian().serialize(&message)?;
    let size = serialized_data.len() as u16;
    let bytes_quantity = size.to_be_bytes();
    writer.write_all(&bytes_quantity)?;
    writer.write_all(serialized_data.as_ref())?;

    Ok(())

}

pub (crate) fn receive_message<Response: Message + de::DeserializeOwned, R:Read>( mut reader: R) 
    -> Result< Box<Response>, SHTCPError> {

        let mut bytes = [0u8, 2];
        reader.read_exact(&mut bytes)?;

        let message_type = u16::from_be_bytes(bytes);

        if message_type != MESSAGE_TYPE {
            return SHTCPError::WrongMessageType(message_type);
        }


}