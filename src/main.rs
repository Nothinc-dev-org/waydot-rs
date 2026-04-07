mod app;
mod clipboard;
mod data;
mod dbus;
mod input;
mod search;
mod ui;

use gtk::prelude::*;

fn main() {
    let app = app::build_app();
    app.run();
}
