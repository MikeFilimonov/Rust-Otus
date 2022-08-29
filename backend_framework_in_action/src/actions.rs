use crate::models::{Device, Room};
use deadpool_postgres::Client;
use std::io::{self, Error as IOError};
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_full_report(client: &Client) -> Result<Vec<Device>, IOError> {
    let query = match client
        .prepare("select * from devices order by room_id")
        .await
    {
        Ok(result) => result,
        Err(e) => panic!("Failed to get the full report due to {e}"),
    };

    let devices = client
        .query(&query, &[])
        .await
        .expect("Failed to get the full report")
        .iter()
        .map(|row| Device::from_row_ref(row).unwrap())
        .collect::<Vec<Device>>();

    Ok(devices)
}

pub async fn add_room(client: &Client, home_id: i32, room_name: String) -> Result<Room, IOError> {
    let query = match client.prepare("insert into rooms (smart_home_id, title) values ($1, $2) returning id, title, smart_home_id")
        .await{
            Ok(result) => result,
            Err(e) => panic!("Failed to insert the record of a new room due to {e}")
        };

    client
        .query(&query, &[&home_id, &room_name])
        .await
        .expect("Failed to create a new room")
        .iter()
        .map(|row| Room::from_row_ref(row).unwrap())
        .collect::<Vec<Room>>()
        .pop()
        .ok_or_else(|| IOError::new(io::ErrorKind::Other, "Failed to create a room"))
}

pub async fn remove_room(client: &Client, room_id: i32) -> Result<Room, IOError> {
    let query = match client
        .prepare("delete from rooms where id = $1 returning id, title, smart_home_id")
        .await
    {
        Ok(result) => result,
        Err(e) => {
            panic!("Failed to delete the record of the room with the given id because of {e}")
        }
    };

    client
        .query(&query, &[&room_id])
        .await
        .expect("Failed to remove the room with the given id")
        .iter()
        .map(|row| Room::from_row_ref(row).unwrap())
        .collect::<Vec<Room>>()
        .pop()
        .ok_or_else(|| IOError::new(io::ErrorKind::InvalidData, "Failed to remove the room"))
}
pub async fn get_room_list(client: &Client) -> Result<Vec<Room>, IOError> {
    let query = match client.prepare("select * from rooms order by id desc").await {
        Ok(result) => result,
        Err(e) => panic!("Failed to get the list of rooms on executing a query because of {e}"),
    };

    let rooms = client
        .query(&query, &[])
        .await
        .expect("Failed to get the room list")
        .iter()
        .map(|row| Room::from_row_ref(row).unwrap())
        .collect::<Vec<Room>>();

    Ok(rooms)
}

pub async fn add_device(
    client: &Client,
    device_name: String,
    home_id: i32,
    room_id: i32,
) -> Result<Device, IOError> {
    let query = match
        client.prepare("insert into devices (title, smart_home_id, room_id) values ($1, $2, $3) returning id, title, smart_home_id, room_id")
        .await{
            Ok(result) => result,
            Err(e) => panic!("Failed to add a record to the devices table due to {e}")
        };

    client
        .query(&query, &[&device_name, &home_id, &room_id])
        .await
        .expect("Failed to create a new device")
        .iter()
        .map(|row| Device::from_row_ref(row).unwrap())
        .collect::<Vec<Device>>()
        .pop()
        .ok_or_else(|| IOError::new(io::ErrorKind::Other, "Failed to create a device"))
}
pub async fn remove_device(
    client: &Client,
    device_id: i32,
    room_id: i32,
) -> Result<Device, IOError> {
    let query = match client
        .prepare("delete from devices where id = $1 and room_id = $2 returning id, title, room_id")
        .await
    {
        Ok(result) => result,
        Err(e) => panic!("Failed to delete the record from the devices table due to {e}"),
    };
    client
        .query(&query, &[&device_id, &room_id])
        .await
        .expect("Failed to remove the device")
        .iter()
        .map(|row| Device::from_row_ref(row).unwrap())
        .collect::<Vec<Device>>()
        .pop()
        .ok_or_else(|| IOError::new(io::ErrorKind::Other, "Failed to remove the device"))
}
pub async fn get_device_list(client: &Client, room_id: i32) -> Result<Vec<Device>, IOError> {
    let query = match client.prepare("select * from devices where room_id=$1 order by id").await {
        Ok(result) => result,
        Err(e) => panic!("Failed to get the list of devices in room {room_id} on executing a query because of {e}")
    };

    let devices = client
        .query(&query, &[&room_id])
        .await
        .expect("Failed to get the device list")
        .iter()
        .map(|row| Device::from_row_ref(row).unwrap())
        .collect::<Vec<Device>>();

    Ok(devices)
}
