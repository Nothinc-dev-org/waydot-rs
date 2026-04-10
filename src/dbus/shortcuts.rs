use gtk::gio;
use gtk::prelude::*;
use libadwaita as adw;

use crate::ui;

pub fn register_shortcuts(app: &adw::Application, clipboard_accel: &str, emoji_accel: &str) {
    let show_clipboard = gio::SimpleAction::new("show-clipboard", None);
    let app_weak = app.downgrade();
    show_clipboard.connect_activate(move |_, _| {
        if let Some(app) = app_weak.upgrade() {
            show_tab(&app, "clipboard");
        }
    });
    app.add_action(&show_clipboard);
    app.set_accels_for_action("app.show-clipboard", &[clipboard_accel]);

    let show_emojis = gio::SimpleAction::new("show-emojis", None);
    let app_weak = app.downgrade();
    show_emojis.connect_activate(move |_, _| {
        if let Some(app) = app_weak.upgrade() {
            show_tab(&app, "emojis");
        }
    });
    app.add_action(&show_emojis);
    app.set_accels_for_action("app.show-emojis", &[emoji_accel]);
}

pub fn show_tab(app: &adw::Application, tab: &str) {
    ui::switch_to_tab(tab);
    if !present_window(app) {
        app.activate();
    }
}

pub fn toggle_window(app: &adw::Application) {
    if let Some(window) = app
        .active_window()
        .or_else(|| app.windows().into_iter().next())
    {
        if window.is_visible() {
            window.set_visible(false);
        } else {
            window.present();
        }
    } else {
        app.activate();
    }
}

pub fn present_window(app: &adw::Application) -> bool {
    if let Some(window) = app
        .active_window()
        .or_else(|| app.windows().into_iter().next())
    {
        window.present();
        true
    } else {
        false
    }
}
