use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};

use crate::icon::Icon;
use crate::style::DialogStyle;
use crate::style::OkCancel;

/// A builder struct used for configuring a [MessageBox](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxa).
/// Uses the MessageBoxA function under the hood.
///
/// From the official Windows documentation:
///
/// "Displays a modal dialog box that contains a system icon, a set of buttons,
/// and a brief application-specific message, such as status or error information.
/// The message box returns an integer value that indicates which button the user clicked."
#[derive(Debug, Default, PartialEq)]
pub struct WinDialog<T = OkCancel>
where
    T: DialogStyle,
{
    /// The content of the message box header. Passing nothing results in
    /// rendering a default header. Passing an empty string results in no header.
    header: Option<String>,

    /// The body text of the message box.
    content: String,

    /// The icon that you want to display. Providing no icon results in no icon
    /// being displayed.
    icon: Option<Icon>,

    /// Determines the button layout for the message box. See the stucts [crate::style]
    /// for the available options.
    style: T,

    /// A pointer to a parent window. Its not expected to be needed for the typical
    /// use cases of this crate, but is included here for completeness.
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
    /// Sets custom content for the message box header. Passing nothing results in
    /// rendering a default header. Passing an empty string results in no header.
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Set an [Icon] for the dialog box.
    pub fn with_icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// A handle to the owner window of the message box to be created.
    /// If this parameter is [None], the message box has no owner window.
    /// In most cases that this crate is designed for, you wouldn't need this parameter,
    /// but it is included for completeness.
    pub fn with_handle(mut self, handle: impl Into<HWND>) -> Self {
        self.window_handle = Some(handle.into());
        self
    }

    /// Indicate which set of actions that you want the user to have. Check the available
    /// options in [crate::style].
    pub fn with_style<N>(self, style: N) -> WinDialog<N>
    where
        N: DialogStyle,
    {
        WinDialog::<N> {
            header: self.header,
            content: self.content,
            style,
            window_handle: self.window_handle,
            icon: self.icon,
        }
    }

    /// Display the dialog and convert results into proper [Result] type.
    /// This is a synchronous action.
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

        let result = unsafe {
            MessageBoxA(
                None,
                content_ptr,
                header_ptr.as_ref(),
                self.style.into() | self.icon.map(MESSAGEBOX_STYLE::from).unwrap_or_default(),
            )
        };

        T::Return::try_from(result)
    }
}
