use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
enum DataTransferError{
    #[error("Failed during data transfer: {0}")]
    Io(#[from] io::Error)

}

#[derive(Debug, Error)]
pub enum ConnectionError {
    
    #[error("Failed during handshake: {0}")]
    HandshakeFailure(#[from] io::Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error)
}

#[derive(Debug, Error)]
pub enum OtherError {
    #[error("Bad encoding")]
    BadEncoding
}

#[derive(Debug, Error)]
pub enum RequestError{
    #[error(transparent)]
    BadRequest(#[from] DataTransferError)
}

#[derive(Debug, Error)]
pub enum ResponseError{
    #[error(transparent)]
    BadEncoding(#[from] OtherError),
    Io(#[from] DataTransferError)
}
    
}