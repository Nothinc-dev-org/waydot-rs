use gtk::gio;
use gtk::prelude::*;
use libadwaita as adw;

pub fn register_global_shortcut(app: &adw::Application, accelerator: &str) {
    app.set_accels_for_action("app.toggle", &[accelerator]);

    let toggle_action = gio::SimpleAction::new("toggle", None);
    let app_weak = app.downgrade();
    toggle_action.connect_activate(move |_, _| {
        if let Some(app) = app_weak.upgrade() {
            toggle_window(&app);
        }
    });
    app.add_action(&toggle_action);
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
