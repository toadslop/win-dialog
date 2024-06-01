use win_dialog::{style, Icon, WinDialog};
use windows::Win32::Foundation::HWND;

fn main() {
    let res =
        WinDialog::new("We encountered an error during installation. What would you like to do?")
            .with_style(style::OkCancel)
            .with_icon(Icon::Hand)
            .with_handle(HWND::default())
            .with_help_button()
            .show()
            .unwrap();
    println!("{res:?}");
}
