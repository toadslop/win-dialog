use win_dialog::{style::AbortRetryIgnore, Icon, WinDialog};

fn main() {
    let res = WinDialog::new("boogaloo")
        .with_style(AbortRetryIgnore)
        .with_icon(Icon::Hand)
        .show()
        .unwrap();

    dbg!(res);
}
