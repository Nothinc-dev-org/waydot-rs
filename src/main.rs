mod app;
mod clipboard;
mod config;
mod data;
mod dbus;
mod emoji_history;
mod input;
mod search;
mod system;
mod tray;
mod ui;

use gtk::prelude::*;

fn main() {
    let app = app::build_app();
    app.run();
}
