use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_DEFBUTTON2, MB_DEFBUTTON3, MB_DEFBUTTON4, MB_HELP, MESSAGEBOX_STYLE,
};

use crate::icon::Icon;
use crate::style::DialogStyle;
use crate::style::{
    AbortRetryIgnore, CancelRetryContinue, OkCancel, RetryCancel, YesNo, YesNoCancel,
};

type ShowReturn<T> = crate::Result<<T as DialogStyle>::Return>;

/// A builder struct used for configuring a [MessageBox](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxa).
/// Uses the MessageBoxA function under the hood.
///
/// From the official Windows documentation:
///
/// "Displays a modal dialog box that contains a system icon, a set of buttons,
/// and a brief application-specific message, such as status or error information.
/// The message box returns an integer value that indicates which button the user clicked."
///
/// The default button const generic
#[derive(Debug, Default, PartialEq)]
pub struct WinDialog<T = OkCancel, const DEFAULT_BUTTON: i32 = 0>
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

    default_button: MESSAGEBOX_STYLE,
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
    pub fn with_handle(self, handle: impl Into<HWND>) -> WinDialogWithParent<T> {
        WinDialogWithParent {
            inner: self,
            window_handle: handle.into(),
            ..Default::default()
        }
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
            icon: self.icon,
            default_button: self.default_button,
        }
    }

    /// Display the dialog and convert results into proper [Result] type.
    /// This is a synchronous action.
    pub fn show(self) -> ShowReturn<T> {
        self.show_inner(Default::default())
    }

    fn show_inner(self, help_button: MESSAGEBOX_STYLE) -> crate::Result<T::Return> {
        let content = CString::new(self.content.to_string())?;
        let content_ptr = PCSTR::from_raw(content.as_ptr() as *const u8);

        let header_ptr = if let Some(header) = self.header {
            let cstr_header = CString::new(header)?;
            let header_ptr = PCSTR::from_raw(cstr_header.as_ptr() as *const u8);
            Some(header_ptr)
        } else {
            None
        };

        let icon = self.icon.map(MESSAGEBOX_STYLE::from).unwrap_or_default();
        let default_button = self.default_button;

        let result = unsafe {
            MessageBoxA(
                None,
                content_ptr,
                header_ptr.as_ref(),
                self.style.into() | icon | help_button | default_button,
            )
        };

        T::Return::try_from(result)
    }
}

impl WinDialog<OkCancel> {
    pub fn set_default_cancel(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialog<AbortRetryIgnore> {
    pub fn set_default_retry(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }

    pub fn set_default_ignore(mut self) -> Self {
        self.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialog<YesNoCancel> {
    pub fn set_default_no(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }

    pub fn set_default_cancel(mut self) -> Self {
        self.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialog<YesNo> {
    pub fn set_default_no(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialog<RetryCancel> {
    pub fn set_default_cancel(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialog<CancelRetryContinue> {
    pub fn set_default_retry(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }

    pub fn set_default_continue(mut self) -> Self {
        self.default_button = MB_DEFBUTTON3;
        self
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct WinDialogWithParent<T>
where
    T: DialogStyle,
{
    /// The inner message box configuration.
    inner: WinDialog<T>,
    /// A pointer to a parent window. Its not expected to be needed for the typical
    /// use cases of this crate, but is included here for completeness.
    window_handle: HWND,

    /// Indicates whether this message box should display a help button.
    show_help_button: bool,
}

impl<T> WinDialogWithParent<T>
where
    T: DialogStyle,
{
    pub fn with_help_button(mut self) -> Self {
        self.show_help_button = true;
        self
    }

    /// Sets custom content for the message box header. Passing nothing results in
    /// rendering a default header. Passing an empty string results in no header.
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.inner.header = Some(header.into());
        self
    }

    /// Set an [Icon] for the dialog box.
    pub fn with_icon(mut self, icon: impl Into<Icon>) -> Self {
        self.inner.icon = Some(icon.into());
        self
    }

    pub fn show(self) -> ShowReturn<T> {
        let help_button = match self.show_help_button {
            true => MB_HELP,
            false => MESSAGEBOX_STYLE::default(),
        };

        self.inner.show_inner(help_button)
    }

    /// Indicate which set of actions that you want the user to have. Check the available
    /// options in [crate::style].
    pub fn with_style<N>(self, style: N) -> WinDialogWithParent<N>
    where
        N: DialogStyle,
    {
        WinDialogWithParent {
            inner: WinDialog::<N> {
                header: self.inner.header,
                content: self.inner.content,
                style,
                icon: self.inner.icon,
                default_button: self.inner.default_button,
            },
            window_handle: self.window_handle,
            show_help_button: self.show_help_button,
        }
    }
}

impl WinDialogWithParent<OkCancel> {
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }

    pub fn set_default_cancel(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialogWithParent<AbortRetryIgnore> {
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON4;
        self
    }

    pub fn set_default_retry(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }

    pub fn set_default_ignore(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialogWithParent<YesNoCancel> {
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON4;
        self
    }

    pub fn set_default_no(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }

    pub fn set_default_cancel(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialogWithParent<YesNo> {
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }

    pub fn set_default_no(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialogWithParent<RetryCancel> {
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }

    pub fn set_default_cancel(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialogWithParent<CancelRetryContinue> {
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON4;
        self
    }

    pub fn set_default_retry(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }

    pub fn set_default_continue(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }
}
