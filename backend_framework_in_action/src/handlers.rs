use actix_web::{web, HttpResponse, Responder};
use crate::actions;
use crate::errors::AppError;
use crate::models::{Status, RoomContainter};
use deadpool_postgres::{Pool, Client};

pub async fn howdy() -> impl Responder{
    HttpResponse::Ok()
    .json(Status{ status: "Ok".to_string()})
}

pub async fn get_room_list(pg_pool: web::Data<Pool>) -> impl Responder{
    
    let client: Client = pg_pool
        .get()
        .await
        .expect("Failed to connect to the DB");

    let result = actions::get_room_list(&client).await;

    match result {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}

pub async fn get_device_list(pg_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder{
    
    let client: Client = pg_pool
        .get()
        .await
        .expect("Failed to connect to the DB");

    let result = actions::get_device_list(&client, path.0).await;

    match result {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}

pub async fn add_room(pg_pool: web::Data<Pool>, json: web::Json<RoomContainter>) -> impl Responder{

    let client: Client = pg_pool
    .get()
    .await
    .expect("Failed to connect to the DB");

    let result = actions::add_room(&client, json.smart_home_id, json.name.clone()).await;

    match result {
        Ok(new_room) => HttpResponse::Ok().json(new_room),
        Err(_) => HttpResponse::InternalServerError().into()
    }  

}