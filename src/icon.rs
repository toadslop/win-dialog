use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONASTERISK, MB_ICONERROR, MB_ICONEXCLAMATION, MB_ICONHAND, MB_ICONINFORMATION,
    MB_ICONQUESTION, MB_ICONSTOP, MB_ICONWARNING, MESSAGEBOX_STYLE,
};

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
    /// A question mark in a blue circle.
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
            Icon::Question => MB_ICONQUESTION,
            Icon::Stop => MB_ICONSTOP,
            Icon::Error => MB_ICONERROR,
            Icon::Hand => MB_ICONHAND,
        }
    }
}
