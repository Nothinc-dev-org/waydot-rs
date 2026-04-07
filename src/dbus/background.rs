use gtk::gio;
use gtk::glib::{self, Variant, variant::ToVariant};
use gtk::prelude::*;
use libadwaita as adw;

use crate::app::APPLICATION_ID;

const PORTAL_BUS_NAME: &str = "org.freedesktop.portal.Desktop";
const PORTAL_OBJECT_PATH: &str = "/org/freedesktop/portal/desktop";
const BACKGROUND_INTERFACE: &str = "org.freedesktop.portal.Background";
const REGISTRY_INTERFACE: &str = "org.freedesktop.host.portal.Registry";
const BACKGROUND_STATUS: &str = "Monitoreando el portapapeles";
const BACKGROUND_REASON: &str =
    "Waydot necesita mantenerse activo para registrar el historial del portapapeles.";

pub fn register_host_app(app: &adw::Application) {
    let Some(connection) = app.dbus_connection() else {
        return;
    };

    let options = glib::VariantDict::new(None);
    let params = Variant::tuple_from_iter([APPLICATION_ID.to_variant(), options.end()]);
    if let Err(err) = connection.call_sync(
        Some(PORTAL_BUS_NAME),
        PORTAL_OBJECT_PATH,
        REGISTRY_INTERFACE,
        "Register",
        Some(&params),
        None,
        gio::DBusCallFlags::NONE,
        -1,
        gtk::gio::Cancellable::NONE,
    ) {
        if !err.message().contains("already associated") {
            eprintln!("No se pudo registrar Waydot en el portal host: {err}");
        }
    }
}

pub fn request_background_access(app: &adw::Application) {
    let Some(connection) = app.dbus_connection() else {
        return;
    };

    let options = glib::VariantDict::new(None);
    options.insert_value("handle_token", &"waydot_background".to_variant());
    options.insert_value("reason", &BACKGROUND_REASON.to_variant());
    options.insert_value("autostart", &false.to_variant());
    let params = Variant::tuple_from_iter(["".to_variant(), options.end()]);

    connection.call(
        Some(PORTAL_BUS_NAME),
        PORTAL_OBJECT_PATH,
        BACKGROUND_INTERFACE,
        "RequestBackground",
        Some(&params),
        None,
        gio::DBusCallFlags::NONE,
        -1,
        gtk::gio::Cancellable::NONE,
        |result| {
            if let Err(err) = result {
                eprintln!("No se pudo solicitar acceso de background para Waydot: {err}");
            }
        },
    );
}

pub fn set_background_status(app: &adw::Application) {
    let Some(connection) = app.dbus_connection() else {
        return;
    };

    let options = glib::VariantDict::new(None);
    options.insert_value("message", &BACKGROUND_STATUS.to_variant());
    let params = Variant::tuple_from_iter([options.end()]);

    connection.call(
        Some(PORTAL_BUS_NAME),
        PORTAL_OBJECT_PATH,
        BACKGROUND_INTERFACE,
        "SetStatus",
        Some(&params),
        None,
        gio::DBusCallFlags::NONE,
        -1,
        gtk::gio::Cancellable::NONE,
        |result| {
            if let Err(err) = result {
                if !err.message().contains("Only sandboxed applications") {
                    eprintln!("No se pudo publicar el estado de background de Waydot: {err}");
                }
            }
        },
    );
}
