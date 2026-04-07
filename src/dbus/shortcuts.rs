use gtk::gio;
use gtk::prelude::*;
use libadwaita as adw;

pub fn register_global_shortcut(app: &adw::Application) {
    app.set_accels_for_action("app.toggle", &["<Super>period"]);

    let toggle_action = gio::SimpleAction::new("toggle", None);
    let app_weak = app.downgrade();
    toggle_action.connect_activate(move |_, _| {
        if let Some(app) = app_weak.upgrade() {
            if let Some(window) = app.active_window() {
                if window.is_visible() {
                    window.set_visible(false);
                } else {
                    window.present();
                }
            }
        }
    });
    app.add_action(&toggle_action);
}
