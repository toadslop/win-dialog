mod dialog;
mod error;
mod icon;
mod modality;
pub mod style;

// pub use dialog::AnyResponse;
pub use dialog::WinDialog;
pub use error::Error;
pub type Result<T = style::OkCancelResponse> = std::result::Result<T, crate::error::Error>;
pub use icon::Icon;
