use crate::models::{SmartHome, Room, Device};
use actix_web::Error;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io::{self, Error as IOError};

pub async fn get_full_report(client: &Client){todo!()}

pub async fn add_room(client: &Client, home_id: i32, room_name: String)-> Result<Room, IOError> {

    let query = client.prepare("insert into rooms (smart_home_id, title) values ($1, $2) returning id, title, smart_home_id")
        .await
        .unwrap();

    client.query(&query, &[&home_id, &room_name])
    .await
    .expect("Failed to create a new room")
    .iter()
    .map(|row| Room::from_row_ref(row).unwrap())
    .collect::<Vec<Room>>()
    .pop()
    .ok_or(IOError::new(io::ErrorKind::Other, "Failed to create a room"))

}


pub async fn remove_room(room: &Room){todo!()}
pub async fn get_room_list(client: &Client) -> Result<Vec<Room>, IOError>{

    let query = match client.prepare("select * from rooms order by id desc").await {
        Ok(result) => result,
        Err(e) => panic!("Failed to get the list of rooms on executing a query because of {e}")
    };

    let rooms = client.query(&query, &[])
        .await
        .expect("Failed to get the room list")
        .iter()
        .map(|row| Room::from_row_ref(row).unwrap())
        .collect::<Vec<Room>>();

    Ok(rooms)

}

pub async fn add_device(){todo!()}
pub async fn remove_device(){todo!()}
pub async fn get_device_list(client: &Client, room_id: i32)-> Result<Vec<Device>, IOError>{
    
    let query = match client.prepare("select * from devices where room_id=$1 order by id").await {
        Ok(result) => result,
        Err(e) => panic!("Failed to get the list of devices in room {room_id} on executing a query because of {e}")
    };

    let devices = client.query(&query, &[&room_id])
        .await
        .expect("Failed to get the device list")
        .iter()
        .map(|row| Device::from_row_ref(row).unwrap())
        .collect::<Vec<Device>>();

    Ok(devices)


}