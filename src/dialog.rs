use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_DEFAULT_DESKTOP_ONLY, MB_DEFBUTTON1, MB_DEFBUTTON2, MB_DEFBUTTON3,
    MB_DEFBUTTON4, MB_HELP, MB_RIGHT, MB_RTLREADING, MB_SERVICE_NOTIFICATION, MB_SETFOREGROUND,
    MB_TOPMOST, MESSAGEBOX_STYLE,
};

use crate::icon::Icon;
use crate::modality::Modality;
use crate::style::DialogStyle;
use crate::style::{
    AbortRetryIgnore, CancelRetryContinue, OkCancel, RetryCancel, YesNo, YesNoCancel,
};

/// Alias used to indicate the common return type for the two [WinDialog] and [WinDialogWithParent].
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

    /// Indicates which button is by default selected (i.e. if the user pressed 'enter'
    /// without doing anything else, which button would be pressed)
    default_button: MESSAGEBOX_STYLE,

    /// Indicates the modality of the box.
    modality: Modality,

    /// Same as desktop of the interactive window station. See [WinDialog::set_default_desktop_only]
    default_desktop_only: bool,

    /// Will display text in right-justified fashion.
    right_justify_text: bool,

    /// Will display in right-to-left style
    right_to_left_reading: bool,

    /// The message box will become the foreground window.
    foreground: bool,

    /// The message box will be created with the WS_EX_TOPMOST window style.
    topmost: bool,

    /// The caller is a service notifying the user of an event.
    is_service_notification: bool,
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
    /// If you don't call this method and provide a handle to the owner window,
    /// the Message Box will have no parent window.
    ///
    /// Attaching a parent window will allow you to add an extra 'help' button
    /// to the message box. See [WinDialogWithParent::with_help_button] for more
    /// information.
    pub fn set_parent_window(mut self, handle: impl Into<HWND>) -> WinDialogWithParent<T> {
        self.is_service_notification = false;
        WinDialogWithParent {
            inner: self,
            window_handle: handle.into(),
            ..Default::default()
        }
    }

    /// Indicate the modality of the dialog box. See [Modality] for the options.
    pub fn set_modality(mut self, modality: Modality) -> Self {
        self.modality = modality;
        self
    }

    /// Same as desktop of the interactive window station. For more information, see
    /// [Window Stations](https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations).
    /// If the current input desktop is not the default desktop, the Message Box does not return until the
    /// user switches to the default desktop.
    pub fn set_default_desktop_only(mut self) -> Self {
        self.default_desktop_only = true;
        self
    }

    /// Set the text to right-justify style.
    pub fn set_right_justify(mut self) -> Self {
        self.right_justify_text = true;
        self
    }

    /// Displays message and caption text using right-to-left reading order on Hebrew and Arabic systems.
    pub fn set_right_to_left_reading(mut self) -> Self {
        self.right_to_left_reading = true;
        self
    }

    /// The message box becomes the foreground window. Internally, the system calls the
    /// [SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow) function for the message box.
    pub fn set_foreground(mut self) -> Self {
        self.foreground = true;
        self
    }

    /// The message box is created with the WS_EX_TOPMOST window style.
    pub fn set_topmost(mut self) -> Self {
        self.topmost = true;
        self
    }

    /// The caller is a service notifying the user of an event. The function displays a message
    /// box on the current active desktop, even if there is no user logged on to the computer.

    /// Terminal Services: If the calling thread has an impersonation token, the function directs
    /// the message box to the session specified in the impersonation token.
    ///
    /// If this function is set, the window must not have a parent. This is so that the message box
    /// can appear on a desktop other than the desktop corresponding to the parent window.
    /// For this reason, if you call [WinDialog::set_parent_window], this parameter will be unset.
    ///
    /// For information on security considerations in regard to using this flag, see
    /// [Interactive Services](https://learn.microsoft.com/en-us/windows/win32/services/interactive-services).
    /// In particular, be aware that this flag can produce interactive content on a locked desktop and
    /// should therefore be used for only a very limited set of scenarios, such as resource exhaustion.
    pub fn make_service_notification(mut self) -> Self {
        self.is_service_notification = true;
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
            foreground: self.foreground,
            right_to_left_reading: self.right_to_left_reading,
            icon: self.icon,
            default_button: self.default_button,
            modality: self.modality,
            topmost: self.topmost,
            is_service_notification: self.is_service_notification,
            default_desktop_only: self.default_desktop_only,
            right_justify_text: self.right_justify_text,
        }
    }

    /// Display the dialog and convert results into proper [Result] type.
    /// This is a synchronous action.
    pub fn show(self) -> ShowReturn<T> {
        self.show_inner(Default::default())
    }

    /// Converts the Rust types to their C counterparts and invokes the MessageBox
    /// api.
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
        let default_deskop_only = match self.default_desktop_only {
            true => MB_DEFAULT_DESKTOP_ONLY,
            false => MESSAGEBOX_STYLE::default(),
        };
        let right_justify = match self.right_justify_text {
            true => MB_RIGHT,
            false => MESSAGEBOX_STYLE::default(),
        };
        let right_to_left_reading = match self.right_to_left_reading {
            true => MB_RTLREADING,
            false => MESSAGEBOX_STYLE::default(),
        };

        let foreground = match self.foreground {
            true => MB_SETFOREGROUND,
            false => MESSAGEBOX_STYLE::default(),
        };

        let topmost = match self.topmost {
            true => MB_TOPMOST,
            false => MESSAGEBOX_STYLE::default(),
        };

        let is_service_notif = match self.is_service_notification {
            true => MB_SERVICE_NOTIFICATION,
            false => MESSAGEBOX_STYLE::default(),
        };

        let result = unsafe {
            MessageBoxA(
                None,
                content_ptr,
                header_ptr.as_ref(),
                self.style.into()
                    | icon
                    | help_button
                    | default_button
                    | default_deskop_only
                    | right_justify
                    | right_to_left_reading
                    | foreground
                    | topmost
                    | is_service_notif,
            )
        };

        T::Return::try_from(result)
    }
}

impl WinDialog<OkCancel> {
    /// Make [crate::style::OkCancelResponse::Cancel] the default response,
    pub fn set_default_cancel(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialog<AbortRetryIgnore> {
    /// Make [crate::style::AbortRetryIgnoreResponse::Retry] the default response,
    pub fn set_default_retry(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }

    /// Make [crate::style::AbortRetryIgnoreResponse::Ignore] the default response,
    pub fn set_default_ignore(mut self) -> Self {
        self.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialog<YesNoCancel> {
    /// Make [crate::style::YesNoCancelResponse::No] the default response,
    pub fn set_default_no(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }

    /// Make [crate::style::YesNoCancelResponse::Cancel] the default response.
    pub fn set_default_cancel(mut self) -> Self {
        self.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialog<YesNo> {
    /// Make [crate::style::YesNoResponse::No] the default response.
    pub fn set_default_no(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialog<RetryCancel> {
    /// Make [crate::style::RetryCancelResponse::Cancel] the default response.
    pub fn set_default_cancel(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialog<CancelRetryContinue> {
    /// Make [crate::style::CancelRetryContinueResponse::Retry] the default response.
    pub fn set_default_retry(mut self) -> Self {
        self.default_button = MB_DEFBUTTON2;
        self
    }

    /// Make [crate::style::CancelRetryContinueResponse::Continue] the default response.
    pub fn set_default_continue(mut self) -> Self {
        self.default_button = MB_DEFBUTTON3;
        self
    }
}

/// A Message Box with an attached parent window.
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
    /// Adds a Help button to the message box. When the user clicks the Help button
    /// or presses F1, the system sends a [WM_HELP](https://learn.microsoft.com/en-us/windows/win32/shell/wm-help)
    /// message to the parent window.
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

    /// Display the message box.
    pub fn show(self) -> ShowReturn<T> {
        let help_button = match self.show_help_button {
            true => MB_HELP,
            false => MESSAGEBOX_STYLE::default(),
        };

        self.inner.show_inner(help_button)
    }

    /// Indicate the modality of the dialog box. See [Modality] for the options.
    pub fn set_modality(mut self, modality: Modality) -> Self {
        self.inner.modality = modality;
        self
    }

    /// Same as desktop of the interactive window station. For more information, see
    /// [Window Stations](https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations).
    /// If the current input desktop is not the default desktop, the Message Box does not return until the
    /// user switches to the default desktop.
    pub fn set_default_desktop_only(mut self) -> Self {
        self.inner.default_desktop_only = true;
        self
    }

    /// Set the text to right-justify style
    pub fn set_right_justify(mut self) -> Self {
        self.inner.right_justify_text = true;
        self
    }

    /// Displays message and caption text using right-to-left reading order on Hebrew and Arabic systems.
    pub fn set_right_to_left_reading(mut self) -> Self {
        self.inner.right_to_left_reading = true;
        self
    }

    /// The message box becomes the foreground window. Internally, the system calls the
    /// [SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow) function for the message box.
    pub fn set_foreground(mut self) -> Self {
        self.inner.foreground = true;
        self
    }

    /// The message box is created with the WS_EX_TOPMOST window style.
    pub fn set_topmost(mut self) -> Self {
        self.inner.topmost = true;
        self
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
                topmost: self.inner.topmost,
                is_service_notification: false,
                right_to_left_reading: self.inner.right_to_left_reading,
                modality: self.inner.modality,
                icon: self.inner.icon,
                default_button: self.inner.default_button,
                default_desktop_only: self.inner.default_desktop_only,
                right_justify_text: self.inner.right_justify_text,
                foreground: self.inner.foreground,
            },
            window_handle: self.window_handle,
            show_help_button: self.show_help_button,
        }
    }
}

impl WinDialogWithParent<OkCancel> {
    /// Sets the help button as default. Will do nothing if [WinDialogWithParent::show_help_button] has not
    /// been called.
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }

    /// Make [crate::style::OkCancelResponse::Cancel] the default response.
    pub fn set_default_cancel(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialogWithParent<AbortRetryIgnore> {
    /// Sets the help button as default. Will do nothing if [WinDialogWithParent::show_help_button] has not
    /// been called.
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON4;
        self
    }

    /// Make [crate::style::AbortRetryIgnoreResponse::Retry] the default response.
    pub fn set_default_retry(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }

    /// Make [crate::style::AbortRetryIgnoreResponse::Ignore] the default response.
    pub fn set_default_ignore(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialogWithParent<YesNoCancel> {
    /// Sets the help button as default. Will do nothing if [WinDialogWithParent::show_help_button] has not
    /// been called.
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON4;
        self
    }

    /// Make [crate::style::YesNoCancelResponse::No] the default response.
    pub fn set_default_no(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }

    /// Make [crate::style::YesNoCancelResponse::Cancel] the default response.
    pub fn set_default_cancel(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }
}

impl WinDialogWithParent<YesNo> {
    /// Sets the help button as default. Will do nothing if [WinDialogWithParent::show_help_button] has not
    /// been called.
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }

    /// Make [crate::style::YesNoResponse::No] the default response.
    pub fn set_default_no(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialogWithParent<RetryCancel> {
    /// Sets the help button as default. Will do nothing if [WinDialogWithParent::show_help_button] has not
    /// been called.
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }

    /// Make [crate::style::RetryCancelResponse::Cancel] the default response.
    pub fn set_default_cancel(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }
}

impl WinDialogWithParent<CancelRetryContinue> {
    /// Set the default button to cancel.
    pub fn set_default_cancel(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON1;
        self
    }

    /// Set the default button to help.
    pub fn set_default_help(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON4;
        self
    }

    /// Set the default button to retry.
    pub fn set_default_retry(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON2;
        self
    }

    /// Set the default button to continue.
    pub fn set_default_continue(mut self) -> Self {
        self.inner.default_button = MB_DEFBUTTON3;
        self
    }
}
