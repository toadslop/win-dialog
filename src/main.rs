use win_dialog::{AbortRetryIgnore, WinDialog};

fn main() {
    let res = WinDialog::new("boogaloo")
        .with_style(AbortRetryIgnore)
        .show()
        .unwrap();

    dbg!(res);
}
