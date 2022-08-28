use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize)]
pub struct Status{
    pub status: String
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table="smart_home")]
pub struct SmartHome{
    pub id: i32,
    pub title: String

}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table="rooms")]
pub struct Room {
    pub id: i32,
    pub title: String, 
    pub smart_home_id: i32
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table="devices")]
pub struct Device {
    pub id: i32,
    pub title: String,
    pub room_id: i32
}