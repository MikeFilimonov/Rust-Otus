use serde::{de::{self, DeserializeOwned}, Serialize};
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs}
};
use rand::{self, Rng};
use crate::tcp_based_protocol::{
    Message,
    receive_message, send_message,
    tcp_based_protocol_types::SHTCPError
    

};

pub struct TCPClient{
    stream: TcpStream,
}

impl TCPClient{

    fn proceed_handshake(mut stream: TcpStream) -> Result<Self, SHTCPError> {
        
        let data = rand::thread_rng().gen::<[u8;32]>();
        stream.write_all(&data)?;
        
        let mut bytes = [0u8;32];
        stream.read_exact(&mut bytes);

        if bytes != data{
            return Err(SHTCPError::HandshakeFailed);
        }

        Ok(Self{stream})

    }

    pub fn connect<A: ToSocketAddrs> (address: A) 
        -> Result<Self,SHTCPError>{
          
            let stream = TcpStream::connect(address)?;
            Self::proceed_handshake(stream)

        }

pub fn send_request <R: Message + Serialize, S: Message + de::DeserializeOwned> (&mut self, request: R)
    -> Result <Box<S>, SHTCPError>{
        
        send_message(request, &mut self.stream)?;
        let response = receive_message(&mut self.stream)?;
        Ok(response)

    }

}
