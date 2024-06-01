use std::ffi::CString;
use std::time::Duration;
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONEXCLAMATION};

use crate::style::DialogStyle;
use crate::style::OkCancel;

/// Represents the inputs for a Wscript.Shell popup.
#[derive(Debug, Default, PartialEq)]
pub struct WinDialog<T = OkCancel>
where
    T: DialogStyle,
{
    header: Option<String>,
    content: String,
    display_duration: Option<Duration>,
    style: T,
    window_handle: Option<HWND>,
}

impl WinDialog {
    /// Create a new dialog with content only. This will wait indefinitely
    /// for user input and will have a default windows title. It will display
    /// a simple popover with only an Ok button and a close icon in the top right.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: OkCancel,
            ..Default::default()
        }
    }
}

impl<T> WinDialog<T>
where
    T: Default + DialogStyle,
{
    /// Adds a custom header to the dialog.
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// The dialog will automatically close once the duration has passed.
    pub fn with_duration(mut self, duration: impl Into<Duration>) -> Self {
        self.display_duration = Some(duration.into());
        self
    }

    /// A handle to the owner window of the message box to be created.
    /// If this parameter is [None], the message box has no owner window.
    pub fn with_handle(mut self, handle: impl Into<HWND>) -> Self {
        self.window_handle = Some(handle.into());
        self
    }

    /// Indicate which set of actions that you want the user to have.
    pub fn with_style<N>(self, style: N) -> WinDialog<N>
    where
        N: DialogStyle,
    {
        WinDialog::<N> {
            header: self.header,
            content: self.content,
            display_duration: self.display_duration,
            style,
            window_handle: self.window_handle,
        }
    }

    /// Display the dialog and convert results into proper [Result] type.
    pub fn show(self) -> crate::Result<T::Return> {
        let content = CString::new(self.content.to_string())?;
        let content_ptr = PCSTR::from_raw(content.as_ptr() as *const u8);

        let header_ptr = if let Some(header) = self.header {
            let cstr_header = CString::new(header)?;
            let header_ptr = PCSTR::from_raw(cstr_header.as_ptr() as *const u8);
            Some(header_ptr)
        } else {
            None
        };
        // let header = self.header.unwrap_or_default();
        // let cstr_header = CString::new(header)?;
        // let header_ptr = PCSTR::from_raw(cstr_header.as_ptr() as *const u8);

        let result = unsafe {
            MessageBoxA(
                None,
                content_ptr,
                header_ptr.as_ref(),
                T::style_code() | MB_ICONEXCLAMATION,
            )
        };

        T::Return::try_from(result)
    }
}
