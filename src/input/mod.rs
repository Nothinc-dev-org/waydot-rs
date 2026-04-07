use std::process::Command;

use gtk::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Backend {
    Wtype,
    Xdotool,
    ClipboardOnly,
}

fn detect_backend() -> Backend {
    if std::env::var("WAYLAND_DISPLAY").is_ok() && command_exists("wtype") {
        Backend::Wtype
    } else if command_exists("xdotool") {
        Backend::Xdotool
    } else {
        Backend::ClipboardOnly
    }
}

fn command_exists(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn inject_text(text: &str) {
    copy_to_clipboard(text);

    match detect_backend() {
        Backend::Wtype => {
            let _ = Command::new("wtype").arg("--").arg(text).spawn();
        }
        Backend::Xdotool => {
            let _ = Command::new("xdotool")
                .args(["key", "--clearmodifiers", "ctrl+v"])
                .spawn();
        }
        Backend::ClipboardOnly => {}
    }
}

fn copy_to_clipboard(text: &str) {
    if let Some(display) = gtk::gdk::Display::default() {
        display.clipboard().set_text(text);
    }
}
