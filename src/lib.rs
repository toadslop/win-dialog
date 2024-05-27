mod dialog;
mod error;

pub use dialog::DialogResponse;
pub use dialog::DialogStyle;
pub use dialog::WinDialog;
pub use error::Error;
pub type Result = std::result::Result<DialogResponse, crate::error::Error>;
