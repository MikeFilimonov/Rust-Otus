use std::io;
use thiserror::Error;

#[derive (Debug, Error)]
pub enum SHTCPError {

    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to bind the connection: {0}")]
    ConnectionFailedToBind(String),

    #[error ("Bad utf-8 encoding")]
    BadEncoding,

    #[error ("Unknown command: {0}")]
    ImproperCommand(String),

    #[error ("Bad socket address")]
    BadSocketAddress,

    // #[error ("Print usage and exit")]
    // PrintUsageAndExit,

}

pub type RequestResult = Result<(), SHTCPError>;
pub type ResponseResult<T> = Result<T, SHTCPError>;