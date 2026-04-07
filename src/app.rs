use std::cell::RefCell;

use adw::prelude::*;
use gtk::gio;
use libadwaita as adw;

use crate::config::AppConfig;
use crate::dbus;
use crate::system;
use crate::ui::Window;

pub const APPLICATION_ID: &str = "com.nothinc.waydot";

thread_local! {
    static APP_HOLD: RefCell<Option<gio::ApplicationHoldGuard>> = const { RefCell::new(None) };
}

pub fn build_app() -> adw::Application {
    let config = AppConfig::load();
    let app = adw::Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    app.connect_startup(move |app| {
        if let Err(err) = system::ensure_user_desktop_integration() {
            eprintln!("No se pudo registrar la metadata de escritorio de Waydot: {err}");
        }
        keep_running_in_background(app);
        dbus::register_global_shortcut(app, &config.toggle_shortcut);
    });

    app.connect_activate(|app| {
        if dbus::present_window(app) {
            return;
        }
        let window = Window::new(app);
        window.present();

        dbus::activate_or_register(app);
        dbus::register_host_app(app);
        dbus::request_background_access(app);
        dbus::set_background_status(app);
    });

    app
}

fn keep_running_in_background(app: &adw::Application) {
    APP_HOLD.with(|hold| {
        let mut hold = hold.borrow_mut();
        if hold.is_none() {
            *hold = Some(app.hold());
        }
    });
}
