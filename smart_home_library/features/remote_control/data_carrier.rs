use serde::{Deserialize, Serialize};
use std::{error, fmt, iter};
use uuid::Uuid;

use crate::{
  tcp_based_protocol::{
    Message,
    network_consts::
        {TCP_RQST_ID, TCP_RSPNS_ID, TCP_TEXT_MSG_ID},
    
    }  
};

#[derive (Clone, Debug, Deserialize, Serialize)]
pub struct TextMessage {
    data: String,
}

 impl Message for TextMessage { 
    const MESSAGE_TYPE: TCP_TEXT_MSG_ID;
 }

 impl fmt::Display for TextMessage {

    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(formatter, "{}", self.data)
    }

 }

 impl TextMessage {
    
    pub fn new <D: AsRef<str>>(data: D) -> Self{
        Self{
            data: data.as_ref().to_owned()
        }
    }

 }

pub (crate) enum RemoteControlRequestMethod {

    GetDeviceState,
    TurnOn,
    TurnOff

}

#[derive (Clone, Debug, Deserialize, Serialize)]
pub struct RemoteControlRequest{
    pub (crate) command: RequestMethod
}

impl Message for RemoteControlRequest {
    const MESSAGE_TYPE: u16 = TCP_RQST_ID;
}

impl RemoteControlRequest{

    pub fn get_device_state() -> Self{
        Self {
            command: RequestMethod::GetDeviceState
        }
    }

    pub fn turn_off() -> Self {
        Self{
            command: RequestMethod::TurnOff
        }
    }
    
    pub fn turn_on() -> Self{
        Self{
            command: RequestMethod::TurnOn
        }
    }
 
}

pub (crate) enum ResponseData {

    State(String),
    Description(String),
    Error(String)
    Items(Vec<(Uuid, String)>)

}

pub struct RemoteControlResponse{
    pub (crate) data: ResponseData,
}

impl <'a> iter::FromIterator <(Uuid, &'a str)> for RemoteControlResponse {

    fn from_item<T: IntoIterator<Item = (Uuid, &'a str)>>(iter:T) -> Self{
        let data : Vec<(Uuid, String)> = iter.
            into_iter().
            map(|(id, name)| (id, name.to_owned())).
            collect(); 

        Self {data: RemoteControlResponse::Items(data)}
    }

}

impl Message for RemoteControlResponse{
    const MESSAGE_TYPE: u16 = TCP_RSPNS_ID;
}

impl RemoteControlResponse {

pub fn send_state(enabled: bool) ->Self{
    Self{data: ResponseData::State(enabled)}
}  

}