use actix_web::{web, HttpResponse, Responder};
use crate::errors::AppError;
use crate::models::Status;

pub async fn howdy() -> impl Responder{
    HttpResponse::Ok()
    .json(Status{ status: "Ok".to_string()})
}