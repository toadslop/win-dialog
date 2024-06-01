use windows::Win32::UI::WindowsAndMessaging::{
    MB_APPLMODAL, MB_SYSTEMMODAL, MB_TASKMODAL, MESSAGEBOX_STYLE,
};

/// Indicate the modality of the dialog box.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Modality {
    #[default]
    /// The user must respond to the message box before continuing work in the window
    /// identified by the [crate::WinDialog::set_parent_window] method. However, the
    /// user can move to the windows of other threads and work in those windows.
    /// Depending on the hierarchy of windows in the application, the user may be able
    /// to move to other windows within the thread. All child windows of the parent
    /// of the message box are automatically disabled, but pop-up windows are not.
    ///
    /// [Modality::App] is the default.
    App,

    /// Produces the same behavior as [Modality::App] except that the message box has the
    /// WS_EX_TOPMOST style. Use system-modal message boxes to notify the user of serious,
    /// potentially damaging errors that require immediate attention (for example, running
    /// out of memory). This flag has no effect on the user's ability to interact with
    /// windows other than those associated with that set by [crate::WinDialog::set_parent_window].
    Task,

    /// Produces the same behavior as [Modality::App] except that all the top-level windows
    /// belonging to the current thread are disabled if no parent window has been provided
    /// using [crate::WinDialog::set_parent_window]. Use this flag when the calling
    /// application or library does not have a window handle available but still needs to
    /// prevent input to other windows in the calling thread without suspending other threads.
    System,
}

impl From<Modality> for MESSAGEBOX_STYLE {
    fn from(value: Modality) -> Self {
        match value {
            Modality::App => MB_APPLMODAL,
            Modality::Task => MB_TASKMODAL,
            Modality::System => MB_SYSTEMMODAL,
        }
    }
}
