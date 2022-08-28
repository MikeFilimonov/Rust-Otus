use actix_web::{web, HttpResponse, Responder};
use crate::actions;
use crate::errors::AppError;
use crate::models::Status;
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