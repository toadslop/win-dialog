use windows::Win32::UI::WindowsAndMessaging::{
    IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY, IDYES, MB_ABORTRETRYIGNORE,
    MB_CANCELTRYCONTINUE, MB_OK, MB_OKCANCEL, MB_RETRYCANCEL, MB_YESNO, MB_YESNOCANCEL,
    MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// Trait indicating the type of response a type of dialog returns,
/// how to convert the raw response to the concrete return type, and
/// how to convert the type into the style code that Powershell understands.
pub trait DialogStyle: Sized + Default {
    /// The concrete type that this style returns
    type Return: TryFrom<MESSAGEBOX_RESULT, Error = crate::Error>;

    /// How to convert this type into the code that Powershell understands
    fn style_code() -> MESSAGEBOX_STYLE;
}

/// Represents a dialog with just an ok button and a close button. A peculiarity about
/// this type is that clicking the X button and the OK button return the same response code,
/// so only use this dialog for informative purposes, but never to allow the user the chance to
/// make a choice.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OkClose;

impl DialogStyle for OkClose {
    type Return = OkResponse;

    fn style_code() -> MESSAGEBOX_STYLE {
        MB_OK
    }
}

/// The possible return values for the [OkClose] dialog.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OkResponse {
    Ok,
}

impl TryFrom<MESSAGEBOX_RESULT> for OkResponse {
    type Error = crate::Error;

    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        if value == IDOK {
            Ok(OkResponse::Ok)
        } else {
            Err(crate::Error::UnknownResponseCode(value.0))
        }
    }
}

/// Represents a dialog that allows the user to accept a proposed action or reject it.
/// It features an X button in the top right corner. This button returns the same value
/// as clicking 'cancel'.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OkCancel;

impl DialogStyle for OkCancel {
    type Return = OkCancelResponse;

    fn style_code() -> MESSAGEBOX_STYLE {
        MB_OKCANCEL
    }
}

impl TryFrom<MESSAGEBOX_RESULT> for OkCancelResponse {
    type Error = crate::Error;

    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        let converted = if value == IDOK {
            OkCancelResponse::Ok
        } else if value == IDCANCEL {
            OkCancelResponse::Cancel
        } else {
            Err(crate::Error::UnknownResponseCode(value.0))?
        };

        Ok(converted)
    }
}

/// The possible return values for [OkCancelClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OkCancelResponse {
    Ok,
    Cancel,
}

/// Represents a dialog that requests user action in the case of an error. The user may choose
/// to abort the action, retry it, or ignore the error.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AbortRetryIgnore;

impl DialogStyle for AbortRetryIgnore {
    type Return = AbortRetryIgnoreResponse;

    fn style_code() -> MESSAGEBOX_STYLE {
        MB_ABORTRETRYIGNORE
    }
}

/// The possible return values for [AbortRetryIgnore]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbortRetryIgnoreResponse {
    Abort,
    Retry,
    Ignore,
}

impl TryFrom<MESSAGEBOX_RESULT> for AbortRetryIgnoreResponse {
    type Error = crate::Error;

    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        let converted = if value == IDABORT {
            AbortRetryIgnoreResponse::Abort
        } else if value == IDRETRY {
            AbortRetryIgnoreResponse::Retry
        } else if value == IDIGNORE {
            AbortRetryIgnoreResponse::Ignore
        } else {
            Err(crate::Error::UnknownResponseCode(value.0))?
        };

        Ok(converted)
    }
}

/// Represents a dialog where a user input is needed during an ongoing action. The user may accept
/// the next action, reject the action, or cancel the process entirely. It also featuers an X button
/// in the top right, which results in the same response code as 'cancel'.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct YesNoCancel;

impl DialogStyle for YesNoCancel {
    type Return = YesNoCancelResponse;

    fn style_code() -> MESSAGEBOX_STYLE {
        MB_YESNOCANCEL
    }
}

impl TryFrom<MESSAGEBOX_RESULT> for YesNoCancelResponse {
    type Error = crate::Error;

    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        let converted = if value == IDYES {
            YesNoCancelResponse::Yes
        } else if value == IDNO {
            YesNoCancelResponse::No
        } else if value == IDCANCEL {
            YesNoCancelResponse::Cancel
        } else {
            Err(crate::Error::UnknownResponseCode(value.0))?
        };

        Ok(converted)
    }
}

/// Possible responses for [YesNoCancelClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YesNoCancelResponse {
    Yes,
    No,
    Cancel,
}

/// Displays a dialog with only two buttons, yes and no.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct YesNo;

impl DialogStyle for YesNo {
    type Return = YesNoResponse;

    fn style_code() -> MESSAGEBOX_STYLE {
        MB_YESNO
    }
}

/// Possible resonses to [YesNo]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YesNoResponse {
    Yes,
    No,
}

impl TryFrom<MESSAGEBOX_RESULT> for YesNoResponse {
    type Error = crate::Error;

    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        let converted = if value == IDYES {
            YesNoResponse::Yes
        } else if value == IDNO {
            YesNoResponse::No
        } else {
            Err(crate::Error::UnknownResponseCode(value.0))?
        };

        Ok(converted)
    }
}

/// Presents two buttons: retry or cancel. It also has an X button at the top right, which
/// returns the same response as 'cancel'.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RetryCancel;

impl DialogStyle for RetryCancel {
    type Return = RetryCancelResponse;

    fn style_code() -> MESSAGEBOX_STYLE {
        MB_RETRYCANCEL
    }
}

/// Possible responses for [RetryCancelClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RetryCancelResponse {
    Retry,
    Cancel,
}

impl TryFrom<MESSAGEBOX_RESULT> for RetryCancelResponse {
    type Error = crate::Error;

    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        let converted = if value == IDRETRY {
            RetryCancelResponse::Retry
        } else if value == IDCANCEL {
            RetryCancelResponse::Cancel
        } else {
            Err(crate::Error::UnknownResponseCode(value.0))?
        };

        Ok(converted)
    }
}

/// Presents three buttons: retry, cancel, and continue. Continue should indicate skipping
/// a failed action but continuing the overarching process.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CancelRetryContinue;

impl DialogStyle for CancelRetryContinue {
    type Return = CancelRetryContinueResponse;

    fn style_code() -> MESSAGEBOX_STYLE {
        MB_CANCELTRYCONTINUE
    }
}

/// Possile responses to [CancelRetryContinueClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CancelRetryContinueResponse {
    Cancel,
    Retry,
    Continue,
}

impl TryFrom<MESSAGEBOX_RESULT> for CancelRetryContinueResponse {
    type Error = crate::Error;

    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        let converted = if value == IDRETRY {
            CancelRetryContinueResponse::Retry
        } else if value == IDCANCEL {
            CancelRetryContinueResponse::Cancel
        } else if value == IDCONTINUE {
            CancelRetryContinueResponse::Continue
        } else {
            Err(crate::Error::UnknownResponseCode(value.0))?
        };

        Ok(converted)
    }
}
