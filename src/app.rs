use adw::prelude::*;
use libadwaita as adw;

use crate::dbus;
use crate::ui::Window;

pub fn build_app() -> adw::Application {
    let app = adw::Application::builder()
        .application_id("com.nothinc.waydot")
        .build();

    app.connect_startup(|app| {
        dbus::register_global_shortcut(app);
    });

    app.connect_activate(|app| {
        if let Some(window) = app.active_window() {
            window.present();
            return;
        }
        let window = Window::new(app);
        window.present();

        dbus::activate_or_register(app);
    });

    app
}
