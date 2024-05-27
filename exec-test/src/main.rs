use std::time::Duration;

fn main() {
    win_dialog::WinDialog::new("Hello")
        .with_duration(Duration::from_secs(20))
        .show();
}
