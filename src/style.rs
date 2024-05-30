/// Trait indicating the type of response a type of dialog returns,
/// how to convert the raw response to the concrete return type, and
/// how to convert the type into the style code that Powershell understands.
pub trait DialogStyle: Sized + Default {
    /// The concrete type that this style returns
    type Return;

    /// How to convert the raw string response into a the concrete response.
    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return>;

    /// How to convert this type into the code that Powershell understands
    fn style_code() -> usize;
}

/// Represents a dialog with just an ok button and a close button. A peculiarity about
/// this type is that clicking the X button and the OK button return the same response code,
/// so only use this dialog for informative purposes, but never to allow the user the chance to
/// make a choice.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OkClose;

impl DialogStyle for OkClose {
    type Return = OkCloseResponse;

    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return> {
        let code = code_raw
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let response = match code {
            1 => Self::Return::Ok,
            unknown => Err(crate::Error::UnknownResponseCode(unknown))?,
        };

        Ok(response)
    }

    fn style_code() -> usize {
        64
    }
}

/// The possible return values for the [OkClose] dialog.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OkCloseResponse {
    Ok,
}

/// Represents a dialog that allows the user to accept a proposed action or reject it.
/// It features an X button in the top right corner. This button returns the same value
/// as clicking 'cancel'.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OkCancelClose;

impl DialogStyle for OkCancelClose {
    type Return = OkCancelCloseResponse;

    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return> {
        let code = code_raw
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let response = match code {
            1 => Self::Return::Ok,
            2 => Self::Return::Cancel,
            unknown => Err(crate::Error::UnknownResponseCode(unknown))?,
        };

        Ok(response)
    }

    fn style_code() -> usize {
        65
    }
}

/// The possible return values for [OkCancelClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OkCancelCloseResponse {
    Ok,
    Cancel,
}

/// Represents a dialog that requests user action in the case of an error. The user may choose
/// to abort the action, retry it, or ignore the error.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AbortRetryIgnore;

impl DialogStyle for AbortRetryIgnore {
    type Return = AbortRetryIgnoreResponse;

    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return> {
        let code = code_raw
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let response = match code {
            3 => Self::Return::Abort,
            4 => Self::Return::Retry,
            5 => Self::Return::Ignore,
            unknown => Err(crate::Error::UnknownResponseCode(unknown))?,
        };

        Ok(response)
    }

    fn style_code() -> usize {
        66
    }
}

/// The possible return values for [AbortRetryIgnore]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbortRetryIgnoreResponse {
    Abort,
    Retry,
    Ignore
}

/// Represents a dialog where a user input is needed during an ongoing action. The user may accept
/// the next action, reject the action, or cancel the process entirely. It also featuers an X button
/// in the top right, which results in the same response code as 'cancel'.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct YesNoCancelClose;

impl DialogStyle for YesNoCancelClose {
    type Return = YesNoCancelCloseResponse;

    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return> {
        let code = code_raw
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let response = match code {
            6 => Self::Return::Yes,
            7 => Self::Return::No,
            2 => Self::Return::Cancel,
            unknown => Err(crate::Error::UnknownResponseCode(unknown))?,
        };

        Ok(response)
    }

    fn style_code() -> usize {
        67
    }
}

/// Possible responses for [YesNoCancelClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YesNoCancelCloseResponse {
    Yes,
    No,
    Cancel
}

/// Displays a dialog with only two buttons, yes and no.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct YesNo;

impl DialogStyle for YesNo {
    type Return = YesNoResponse;

    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return> {
        let code = code_raw
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let response = match code {
            6 => Self::Return::Yes,
            7 => Self::Return::No,
            unknown => Err(crate::Error::UnknownResponseCode(unknown))?,
        };

        Ok(response)
    }

    fn style_code() -> usize {
        68
    }
}

/// Possible resonses to [YesNo]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YesNoResponse {
    Yes,
    No,
}

/// Presents two buttons: retry or cancel. It also has an X button at the top right, which
/// returns the same response as 'cancel'.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RetryCancelClose;

impl DialogStyle for RetryCancelClose {
    type Return = RetryCancelCloseResponse;

    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return> {
        let code = code_raw
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let response = match code {
            4 => Self::Return::Retry,
            2 => Self::Return::Cancel,
            unknown => Err(crate::Error::UnknownResponseCode(unknown))?,
        };

        Ok(response)
    }

    fn style_code() -> usize {
        69
    }
}

/// Possible responses for [RetryCancelClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RetryCancelCloseResponse {
    Retry,
    Cancel,
}

/// Presents three buttons: retry, cancel, and continue. Continue should indicate skipping
/// a failed action but continuing the overarching process.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CancelRetryContinueClose;

impl DialogStyle for CancelRetryContinueClose {
    type Return = CancelRetryContinueCloseResponse;

    fn convert_response_code(code_raw: &str) -> crate::Result<Self::Return> {
        let code = code_raw
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let response = match code {
            2 => Self::Return::Cancel,
            10 => Self::Return::Retry,
            11 => Self::Return::Continue,
            unknown => Err(crate::Error::UnknownResponseCode(unknown))?,
        };

        Ok(response)
    }

    fn style_code() -> usize {
        70
    }
}

/// Possile responses to [CancelRetryContinueClose]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CancelRetryContinueCloseResponse {
    Cancel,
    Retry,
    Continue,
}
