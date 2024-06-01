mod dialog;
mod error;
mod style;

// pub use dialog::AnyResponse;
pub use dialog::WinDialog;
pub use error::Error;
pub type Result<T = OkCancelResponse> = std::result::Result<T, crate::error::Error>;
pub use style::{
    AbortRetryIgnore, AbortRetryIgnoreResponse, CancelRetryContinue, CancelRetryContinueResponse,
    OkCancel, OkCancelResponse, OkClose, OkResponse, RetryCancel, RetryCancelResponse, YesNo,
    YesNoCancel, YesNoCancelResponse, YesNoResponse,
};
