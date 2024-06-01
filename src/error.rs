use std::ffi::NulError;

/// The possible errors that could occur when showing the message
/// box.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Typically, this error code should never appear unless there
    /// is a bug in this crate or Windows introduced new codes.
    #[error("Dialog returned unknown response code: {0}")]
    UnknownResponseCode(i32),

    /// This error occurs in converting an input string to the [std::ffi::CString]
    /// representation that Windows expects fails.
    #[error("String could not be converted to C-string: {0}")]
    InvalidString(#[from] NulError),
}
