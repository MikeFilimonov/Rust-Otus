use io_via_udp_multithread_flavoured::{consts, DataEmulator};
use std::{
    env,
    net::{SocketAddr, UdpSocket},
    thread,
    time::Duration
};


fn main(){


    let args = env::args();
    let mut args = args.skip(1);
    
    let receiver_address = args
        .next()
        .unwrap_or_else(|| consts::DEFAULT_THERMO_ADDRESS.into());

    println!("Receiver connected at {receiver_address}");
    
    let receiver = receiver_address
        .parse::<SocketAddr>()
        .expect("Invalid udp address. Please try another one.");
    
    let server_address = consts::DEFAULT_UDP_CLIENT;
    let connection = UdpSocket::bind(server_address)
        .expect("Can't bind to the address {server_address}");
    let data_emulator = DataEmulator::default();


    println!("Sending current temperature from {server_address} to {receiver_address} via UDP");

    loop{
        
        let value = data_emulator.emulate();
        let bytes = value.to_be_bytes();
        let sent_data = connection.send_to(&bytes,receiver);
        if let Err(err) = sent_data{
            println!("Failed to share current temperature: {err}")
        }
        thread::sleep(Duration::from_secs(1))

    }


}