use std::{io, process::Command, time::Duration};

#[derive(Debug, Default, PartialEq)]
pub struct WinDialog {
    header: Option<String>,
    content: String,
    display_duration: Option<Duration>,
    style: Option<DialogStyle>,
}

impl WinDialog {
    fn get_param_string(self) -> String {
        let mut params = [Some(format!("\"{}\"", self.content)), None, None, None];

        if let Some(style) = self.style {
            params[3] = Some(format!("\"{}\"", (style as usize)));
            params[2] = Some(self.header.unwrap_or_default());
            params[1] = Some(
                self.display_duration
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
            );
        } else if let Some(header) = self.header {
            params[2] = Some(header);
            params[1] = Some(Default::default());
        } else if let Some(duration) = self.display_duration {
            params[1] = Some(duration.as_secs().to_string());
        }

        let args = params.into_iter().flatten().collect::<Vec<_>>();
        args.join(", ")
    }

    pub fn show(self) -> io::Result<DialogResponse> {
        let command = format!(
            "(New-Object -ComObject Wscript.Shell).popup({})",
            self.get_param_string()
        );

        let output = Command::new("powershell.exe").arg(command).output()?;

        if !output.status.success() {
            todo!();
        }

        let code_raw = match String::from_utf8(output.stdout) {
            Ok(code) => code,
            Err(_) => todo!("error"),
        }
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

        let it = DialogResponse::try_from(code_raw);

        Ok(DialogResponse::Ok)
    }
}

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

impl TryFrom<String> for DialogResponse {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let code = match value.parse::<u8>() {
            Ok(code) => code,
            Err(_) => todo!("return err"),
        };

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
            _ => return Err("err".into()),
        };

        Ok(code)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct WinDialogBuilder {
    header: Option<String>,
    content: String,
    display_duration: Option<Duration>,
    style: Option<DialogStyle>,
}

impl WinDialogBuilder {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn with_duration(mut self, duration: impl Into<Duration>) -> Self {
        self.display_duration = Some(duration.into());
        self
    }

    pub fn with_style(mut self, style: impl Into<DialogStyle>) -> Self {
        self.style = Some(style.into());
        self
    }

    pub fn build(self) -> WinDialog {
        self.into()
    }
}

impl From<WinDialogBuilder> for WinDialog {
    fn from(value: WinDialogBuilder) -> Self {
        Self {
            header: value.header,
            content: value.content,
            display_duration: value.display_duration,
            style: value.style,
        }
    }
}

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
