#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

//! # WinDialog
//!
//! Provides an idiomatic Rust interface on top of Window's Message Box api.
//!
//! ## Overview
//!
//! Window's Message Box api allows you to get input from the user in the form of
//! a set of fixed options displayed as buttons on a simple dialog box. It allows
//! custom header text, body text, and allows a choice of icons, among a few other
//! settings. The [windows crate](https://crates.io/crates/windows) exposes a typesafe
//! interface, but it just a thin wrapper around the raw C-api.
//!
//! This crate provides a further layer on top of this C-wrapper to make interacting
//! with it feel more like native Rust.
//!
//! ## Usage
//!
//! ### Simple Example:
//!
//! ```rust
//! use win_dialog::{style, Icon, WinDialog};
//! use windows::Win32::Foundation::HWND;
//!
//! let res =
//!     WinDialog::new("We encountered an error during installation. What would you like to do?")
//!         .with_header("Installation error")
//!         .with_style(style::AbortRetryIgnore)
//!         .with_icon(Icon::Warning)
//!         .show()
//!         .unwrap();
//! println!("{res:?}");
//! ```
//!

/// Contains the core WinDialog struct builder.
mod dialog;
/// Errors that could occur when rendering the dialog.
mod error;
/// Contains enum modeling the available icons.
mod icon;
/// Enum modeling the modality options available.
mod modality;
/// Traits and marker structs modeling the different styles of dialog box.
pub mod style;

// pub use dialog::AnyResponse;
pub use dialog::{WinDialog, WinDialogWithParent};
pub use error::Error;
/// Custom error type alias for the crate.
pub type Result<T = style::OkCancelResponse> = std::result::Result<T, crate::error::Error>;
pub use icon::Icon;
pub use modality::Modality;
