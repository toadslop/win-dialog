use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONASTERISK, MB_ICONERROR, MB_ICONEXCLAMATION, MB_ICONHAND, MB_ICONINFORMATION,
    MB_ICONSTOP, MB_ICONWARNING, MESSAGEBOX_STYLE,
};

#[cfg(feature = "deprecated")]
use windows::Win32::UI::WindowsAndMessaging::MB_ICONQUESTION;

/// Represents the set of icons available for a message box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Icon {
    /// An exclamation point in a yellow triangle.
    Exclamation,
    /// An alias for [Icon::Exclamation].
    Warning,
    /// The letter 'i' in a blue circle.
    Information,
    /// An alias for [Icon::Information].
    Asterisk,
    #[cfg(feature = "deprecated")]
    #[deprecated]
    /// A question mark in a blue circle.
    ///
    /// ## Deprecation Warning
    ///
    /// According to the official windows documentation:
    ///
    /// "The question-mark message icon is no longer recommended because it does
    /// not clearly represent a specific type of message and because the phrasing
    /// of a message as a question could apply to any message type. In addition,
    /// users can confuse the message symbol question mark with Help information.
    /// Therefore, do not use this question mark message symbol in your message boxes.
    /// The system continues to support its inclusion only for backward compatibility."
    Question,
    /// The letter 'x' in a red circle.
    Stop,
    /// An alias for [Icon::Stop].
    Error,
    /// Despite the name, this is an alias for [Icon::Stop]. It does not
    /// display a hand. This is an idiosyncrasy of Windows.
    Hand,
}

impl From<Icon> for MESSAGEBOX_STYLE {
    fn from(value: Icon) -> Self {
        match value {
            Icon::Exclamation => MB_ICONEXCLAMATION,
            Icon::Warning => MB_ICONWARNING,
            Icon::Information => MB_ICONINFORMATION,
            Icon::Asterisk => MB_ICONASTERISK,
            #[cfg(feature = "deprecated")]
            Icon::Question => MB_ICONQUESTION,
            Icon::Stop => MB_ICONSTOP,
            Icon::Error => MB_ICONERROR,
            Icon::Hand => MB_ICONHAND,
        }
    }
}
