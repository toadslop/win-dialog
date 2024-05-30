mod dialog;
mod error;
mod style;

pub use dialog::AnyResponse;
pub use dialog::WinDialog;
pub use error::Error;
pub type Result<T = AnyResponse> = std::result::Result<T, crate::error::Error>;
pub use style::{
    AbortRetryIgnore, AbortRetryIgnoreResponse, CancelRetryContinueClose,
    CancelRetryContinueCloseResponse, OkCancelClose, OkCancelCloseResponse, OkClose,
    OkCloseResponse, RetryCancelClose, RetryCancelCloseResponse, YesNo, YesNoCancelClose, YesNoCancelCloseResponse,
    YesNoResponse,
};
