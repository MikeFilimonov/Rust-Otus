use actix_web::{error::ResponseError, http::StatusCode, HttpResponse, App};
use std::fmt;


#[derive(Debug)]
pub enum AppErrorKind{
    InvalidRequest,
    NotFound
}

#[derive(Debug)]
pub struct AppError{
    pub text: Option<String>,
    pub cause: Option<String>,
    pub kind: AppErrorKind
}


impl AppError{
     pub fn message(&self) -> String {
        match &*self {
            AppError {
                text: Some(text),
                ..
            } => text.clone(),
            AppError {
                text:None,
                kind: AppErrorKind::NotFound,
                ..} => "Failed to find the requested item".to_string(),
            _ => "An unexpected error has occured".to_string(),
            }
        }
     }
