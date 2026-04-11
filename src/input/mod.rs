#[allow(dead_code)]
mod native_injection;
#[allow(dead_code)]
mod wayland;
#[allow(dead_code)]
mod x11;

use gtk::prelude::*;

use crate::debug;

pub fn copy_text(text: &str) {
    debug::input_log(
        "clipboard",
        format!(
            "publicando seleccion al portapapeles activo; text={text:?}; chars={}",
            text.chars().count()
        ),
    );

    if let Some(display) = gtk::gdk::Display::default() {
        display.clipboard().set_text(text);
    } else {
        debug::input_log(
            "clipboard",
            "no hay Display GTK disponible; no se pudo copiar al portapapeles",
        );
    }
}
