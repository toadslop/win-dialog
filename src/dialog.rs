use std::fmt::Display;
use std::{process::Command, time::Duration};

/// Represents the inputs for a Wscript.Shell popup.
#[derive(Debug, Default, PartialEq)]
pub struct WinDialog {
    header: Option<InputString>,
    content: InputString,
    display_duration: Option<Duration>,
    style: Option<DialogStyle>,
}

impl WinDialog {
    /// Create a new dialog with content only. This will wait indefinitely
    /// for user input and will have a default windows title. It will display
    /// a simple popover with only an Ok button and a close icon in the top right.
    pub fn new(content: impl Into<InputString>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Adds a custom header to the dialog.
    pub fn with_header(mut self, header: impl Into<InputString>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// The dialog will automatically close once the duration has passed.
    pub fn with_duration(mut self, duration: impl Into<Duration>) -> Self {
        self.display_duration = Some(duration.into());
        self
    }

    /// Indicate which set of actions that you want the user to have.
    pub fn with_style(mut self, style: impl Into<DialogStyle>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Formats the params as a comma separated list in the correct order.
    fn get_param_string(self) -> String {
        let mut params = [Some(self.content.to_string()), None, None, None];

        if let Some(style) = self.style {
            params[3] = Some(format!("'{}'", (style as usize)));
            params[2] = Some(self.header.unwrap_or_default().to_string());
            params[1] = Some(
                self.display_duration
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
            );
        } else if let Some(header) = self.header {
            params[2] = Some(header.to_string());
            params[1] = Some(Default::default());
        } else if let Some(duration) = self.display_duration {
            params[1] = Some(duration.as_secs().to_string());
        }

        let args = params.into_iter().flatten().collect::<Vec<_>>();
        args.join(", ")
    }

    /// Display the dialog and convert results into proper [Result] type.
    pub fn show(self) -> crate::Result {
        let command = format!(
            "(New-Object -ComObject Wscript.Shell).popup({})",
            self.get_param_string()
        );

        let output = Command::new("powershell.exe")
            .arg(command)
            .output()
            .map_err(crate::Error::PowershellNotFound)?;

        if !output.status.success() {
            let report = String::from_utf8_lossy(&output.stderr);
            Err(crate::Error::ExecError(report.to_string()))?;
        }

        let code_raw = String::from_utf8(output.stdout).map_err(crate::Error::StdoutDecodeError)?;
        let response = DialogResponse::try_from(&code_raw[0..code_raw.len() - 2])?;
        Ok(response)
    }
}

/// Indicates the response options provided for the user. Note that variants ending in "Close"
/// provide an X icon in the top right corner of the dialog.
#[derive(Debug, Default, PartialEq)]
pub enum DialogStyle {
    #[default]
    OkClose = 64,
    OkCancelClose = 65,
    AbortRetryIgnore = 66,
    YesNoCancelClose = 67,
    YesNo = 68,
    RetryCancelClose = 69,
    CancelRetryContinueClose = 70,
}

/// Represents the possible responses from the user.
#[derive(Debug, PartialEq)]
pub enum DialogResponse {
    Ok,
    Cancel,
    Abort,
    Retry,
    Ignore,
    Yes,
    No,
    Rerun,
    Continue,
}

impl TryFrom<&str> for DialogResponse {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let code = value
            .parse::<u8>()
            .map_err(crate::Error::ParseResponseCodeFailure)?;

        let code = match code {
            1 => Self::Ok,
            2 => Self::Cancel,
            3 => Self::Abort,
            4 => Self::Retry,
            5 => Self::Ignore,
            6 => Self::Yes,
            7 => Self::No,
            10 => Self::Rerun,
            11 => Self::Continue,
            other => Err(crate::Error::UnknownResponseCode(other))?,
        };

        Ok(code)
    }
}

/// A wrapper around [String] to allow a custom default implementation that
/// makes the default value "\""\" rather than "".
#[derive(Debug, PartialEq)]
pub struct InputString(String);

impl InputString {
    fn sanitize(&self) -> String {
        let mut init = String::with_capacity(self.0.len() + 2);
        init.push('"');
        let mut finish = self.0.chars().fold(init, |mut sanitized, char| {
            if char == '\'' || char == '"' {
                sanitized.push('`');
            }
            sanitized.push(char);
            sanitized
        });

        finish.push('"');

        finish
    }
}

impl Default for InputString {
    fn default() -> Self {
        Self("\"\"".into())
    }
}

impl Display for InputString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sanitize())
    }
}

impl<T> From<T> for InputString
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
