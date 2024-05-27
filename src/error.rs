use std::{io, num::ParseIntError, string::FromUtf8Error};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not find powershell")]
    PowershellNotFound(#[source] io::Error),
    #[error("Error executing powershell script:\n{0}")]
    ExecError(String),
    #[error("Failed to decode success response from stdout")]
    StdoutDecodeError(#[source] FromUtf8Error),
    #[error("Failed parse the response code")]
    ParseResponseCodeFailure(#[source] ParseIntError),
    #[error("Dialog returned unknown response code: {0}")]
    UnknownResponseCode(u8),
}
