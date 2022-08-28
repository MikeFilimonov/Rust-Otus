use crate::models::{SmartHome, Room, Device};
use actix_web::Error;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io::Error as IOError;

pub async fn get_full_report(client: &Client){todo!()}

pub async fn add_room(){todo!()}
pub async fn remove_room(room: &Room){todo!()}
pub async fn get_room_list(client: &Client) -> Result<Vec<Room>, IOError>{

    let query = match client.prepare("select * from rooms").await {
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
pub async fn get_device_list(room: &Room){todo!()}