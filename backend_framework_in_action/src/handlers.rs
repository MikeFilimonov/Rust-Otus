use crate::actions;
use actix_web::{web, HttpResponse, Responder};
// use crate::errors::AppError;
use crate::models::{DeviceContainer, RoomContainter, Status};
use deadpool_postgres::{Client, Pool};

pub async fn howdy() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

pub async fn get_full_report(pg_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = pg_pool.get().await.expect("Failed to connect to the DB");

    let result = actions::get_full_report(&client).await;

    match result {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_room_list(pg_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = pg_pool.get().await.expect("Failed to connect to the DB");

    let result = actions::get_room_list(&client).await;

    match result {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_device_list(pg_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = pg_pool.get().await.expect("Failed to connect to the DB");

    let result = actions::get_device_list(&client, path.0).await;

    match result {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn add_room(pg_pool: web::Data<Pool>, json: web::Json<RoomContainter>) -> impl Responder {
    let client: Client = pg_pool.get().await.expect("Failed to connect to the DB");

    let result = actions::add_room(&client, json.smart_home_id, json.name.clone()).await;

    match result {
        Ok(new_room) => HttpResponse::Created().json(new_room),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn remove_room(pg_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = pg_pool.get().await.expect("Failed to connect to the DB");

    let result = actions::remove_room(&client, path.0).await;

    match result {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(_) => HttpResponse::NoContent().into(),
    }
}

pub async fn add_device(
    pg_pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
    json: web::Json<DeviceContainer>,
) -> impl Responder {
    let client = pg_pool.get().await.expect("Failed to connect to the DB");

    let result = actions::add_device(&client, json.name.clone(), json.smart_home_id, path.0).await;

    match result {
        Ok(new_device) => HttpResponse::Created().json(new_device),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn remove_device(
    pg_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let client: Client = pg_pool.get().await.expect("Failed to connect to the DB");

    let result = actions::remove_device(&client, path.1, path.0).await;

    match result {
        Ok(removed_device) => HttpResponse::Ok().json(removed_device),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
