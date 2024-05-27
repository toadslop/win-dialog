use std::time::Duration;

fn main() {
    win_dialog::WinDialogBuilder::new("Hello")
        .with_duration(Duration::from_secs(20))
        .build()
        .show();
}
