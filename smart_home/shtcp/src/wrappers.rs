use std::io;
use thiserror::Error;

#[derive (Debug, Error)]
pub enum ConnectionError {
    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to bind the connection: {0}")]
    ConnectionFaildToBind(String),
    
}

#[derive (Debug, Error)]
pub enum RequestError {
    #[error ("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive (Debug, Error)]
pub enum ResponseError {
    #[error ("IO error: {0}")]
    Io(#[from] io::Error),
    #[error ("Bad encoding")]
    BadEncoding,
}

pub type RequestResult = Result<(), RequestError>;
pub type ResponseResult<T> = Result<T, ResponseError>;