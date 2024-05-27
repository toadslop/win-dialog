use std::time::Duration;

use win_dialog::DialogStyle;

fn main() {
    let response = win_dialog::WinDialog::new("Hello")
        .with_duration(Duration::from_secs(20))
        .with_style(DialogStyle::AbortRetryIgnore)
        .show()
        .unwrap();

    dbg!(response);
}
