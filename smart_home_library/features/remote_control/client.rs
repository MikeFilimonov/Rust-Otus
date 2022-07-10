use std::net::ToSocketAddrs;

use crate::{
    data_carrier,
    tcp_based_protocol::{
        client::TCPClient,
        tcp_based_protocol_types::SHTCPError}
};