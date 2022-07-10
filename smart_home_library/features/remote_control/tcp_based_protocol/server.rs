use serde::{de, Serialize};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs}
};

use crate::tcp_based_protocol::{
    Message, send_message, receive_message,
    tcp_based_protocol_types::{SHTCPError, ResponseResult, RequestResult},
};

pub struct RemoteControlServer{
    listener: TcpListener
}

pub struct TCPConnection{
    stream: TcpStream
}

impl RemoteControlServer{

    fn proceed_handshake_for_tcp (mut stream: TcpStream)
        -> Result<TCPConnection, SHTCPError>{

            let mut bytes = [0u8; 32];
            stream.read_exact(&mut bytes)?;
            stream.write_all(&bytes)?;
            Ok(TCPConnection{stream})

        }

    pub fn bind_via_tcp<A:ToSocketAddrs>(address: A) 
        -> Result<Self, SHTCPError> {

            let listener = TcpListener::bind(address)?;
            Ok(Self{listener})

        }

    pub fn block_incoming_connections_for_tcp(&self) ->
     impl Iterator < Item = Result<TCPConnection, SHTCPError>> + '_ {

        self.listener.incoming().map(|stream| 
            match stream {
              
              Ok(stream) => Self::proceed_handshake_for_tcp(stream),
              Err(error) => Err(SHTCPError::Io(error))

            }
        )

     }
    


}

impl TCPConnection {

    pub fn send_response<M: Message + Serialize> (&mut self, response: M)
    -> RequestResult{
        send_message(response, &mut self.stream)
    }

    pub fn get_client_response<M: Message + de::DeserializeOwned>(&mut self)
        -> ResponseResult<Box<M>> {
            receive_message(&mut self.stream)
        }
    
    pub fn connected_client_address(&self) -> io::Result<SocketAddr>{
        self.stream.peer_addr()
    }

}
