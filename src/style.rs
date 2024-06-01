use windows::Win32::UI::WindowsAndMessaging::{
    IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY, IDYES, MB_ABORTRETRYIGNORE,
    MB_CANCELTRYCONTINUE, MB_OK, MB_OKCANCEL, MB_RETRYCANCEL, MB_YESNO, MB_YESNOCANCEL,
    MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// Trait indicating the type of response style of dialog returns,
/// how to convert the raw response to the concrete return type, and
/// how to convert the type into the style code Windows understands.
pub trait DialogStyle: Sized + Default + Into<MESSAGEBOX_STYLE> {
    /// The concrete type that this style returns
    type Return: TryFrom<MESSAGEBOX_RESULT, Error = crate::Error>;

    /// A helper method to convert to the raw style code. Under the hood,
    /// simply calls [Into]
    fn style_code(self) -> MESSAGEBOX_STYLE {
        self.into()
    }
}

/// Represents a dialog with just an ok button and a close button. A peculiarity about
/// this type is that clicking the X button and the OK button return the same response code,
/// so only use this dialog for informative purposes, but never to allow the user the chance to
/// make a choice.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ok_;

impl DialogStyle for Ok_ {
    type Return = OkResponse;
}

impl From<Ok_> for MESSAGEBOX_STYLE {
    fn from(_: Ok_) -> Self {
        MB_OK
    }
}

/// The possible return values for the [Ok_] dialog.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OkResponse {
    /// The user acknowledged the response.
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
}

impl From<OkCancel> for MESSAGEBOX_STYLE {
    fn from(_: OkCancel) -> Self {
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

/// The possible return values for [OkCancel]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OkCancelResponse {
    /// The user agreed to perform the action described by the message box's content.
    Ok,
    /// The user does not want to perform the action described by the message box's content.
    Cancel,
}

/// Represents a dialog that requests user action in the case of an error. The user may choose
/// to abort the action, retry it, or ignore the error. This is typically used when a sequence
/// of actions are being carried out and one step encountered an error.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AbortRetryIgnore;

impl DialogStyle for AbortRetryIgnore {
    type Return = AbortRetryIgnoreResponse;
}

impl From<AbortRetryIgnore> for MESSAGEBOX_STYLE {
    fn from(_: AbortRetryIgnore) -> Self {
        MB_ABORTRETRYIGNORE
    }
}

/// The possible return values for [AbortRetryIgnore]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbortRetryIgnoreResponse {
    /// The user wants to give up performing the action.
    Abort,
    /// The user wants the action to be performed again.
    Retry,
    /// The user wants to ignore the error but not retry the action.
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

/// Represents a dialog where a user input is needed during an ongoing series of actions. The user may accept
/// the next action, reject the action, or cancel the process entirely. It also featuers an X button
/// in the top right, which results in the same response code as 'cancel'.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct YesNoCancel;

impl DialogStyle for YesNoCancel {
    type Return = YesNoCancelResponse;
}

impl From<YesNoCancel> for MESSAGEBOX_STYLE {
    fn from(_: YesNoCancel) -> Self {
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

/// Possible responses for [YesNoCancel]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YesNoCancelResponse {
    /// The user accepts the proposed action. Proceed to the next step in the series of actions.
    Yes,
    /// The user rejects the prososed action. Continue to the next action.
    No,
    /// The user rejects the proposed action. Do not proceed to the next step.
    Cancel,
}

/// Displays a dialog with only two buttons, yes and no. Used in cases where there is only as single
/// action to be performed.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct YesNo;

impl DialogStyle for YesNo {
    type Return = YesNoResponse;
}

impl From<YesNo> for MESSAGEBOX_STYLE {
    fn from(_: YesNo) -> Self {
        MB_YESNO
    }
}

/// Possible resonses to [YesNo]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YesNoResponse {
    /// The user accepts the proposed action.
    Yes,
    /// The user rejects the proposed action.
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
/// returns the same response as 'cancel'. Use in cases where only a single action occurs
/// rather than a sequence of actions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RetryCancel;

impl DialogStyle for RetryCancel {
    type Return = RetryCancelResponse;
}

impl From<RetryCancel> for MESSAGEBOX_STYLE {
    fn from(_: RetryCancel) -> Self {
        MB_RETRYCANCEL
    }
}

/// Possible responses for [RetryCancel]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RetryCancelResponse {
    /// The user indicated a desire to try the operation again.
    Retry,
    /// The user indicated a desire to abandon the process after a failure.
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
}

impl From<CancelRetryContinue> for MESSAGEBOX_STYLE {
    fn from(_: CancelRetryContinue) -> Self {
        MB_CANCELTRYCONTINUE
    }
}

/// Possile responses to [CancelRetryContinue]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CancelRetryContinueResponse {
    /// The user indicates a desire to abandon the sequences of actions entirely.
    Cancel,
    /// The user indicates a desire to retry a failed action.
    Retry,
    /// The user indicates a desire to perform the next action despite the failure of the previous.
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
