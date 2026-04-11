use crate::debug;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Backend {
    Wayland,
    X11,
    ClipboardOnly,
}

pub(crate) fn inject_text(text: &str) {
    debug::input_log(
        "inject",
        format!(
            "solicitud de insercion nativa recibida; text={text:?}; chars={}",
            text.chars().count()
        ),
    );

    match inject_with_backend(text) {
        Ok(()) => {
            debug::input_log("inject", "insercion nativa completada sin error");
        }
        Err(err) => {
            debug::input_log("inject", format!("insercion nativa fallida: {err}"));
            eprintln!("No se pudo insertar el texto seleccionado: {err}");
        }
    }
}

fn inject_with_backend(text: &str) -> Result<(), String> {
    let backend = detect_backend();
    debug::input_log(
        "backend",
        format!(
            "backend nativo detectado: {backend:?}; WAYLAND_DISPLAY={:?}; DISPLAY={:?}",
            std::env::var_os("WAYLAND_DISPLAY"),
            std::env::var_os("DISPLAY")
        ),
    );

    match backend {
        Backend::Wayland => match super::wayland::inject_text(text) {
            Ok(()) => Ok(()),
            Err(wayland_error) => {
                debug::input_log("backend", format!("backend Wayland fallo: {wayland_error}"));
                if has_display("DISPLAY") {
                    debug::input_log(
                        "backend",
                        "intentando fallback X11 despues del fallo en Wayland",
                    );
                    super::x11::inject_paste().map_err(|x11_error| {
                        format!(
                            "backend Wayland fallo: {wayland_error}; backend X11 tambien fallo: {x11_error}"
                        )
                    })
                } else {
                    Err(wayland_error)
                }
            }
        },
        Backend::X11 => super::x11::inject_paste(),
        Backend::ClipboardOnly => {
            debug::input_log(
                "backend",
                "sin backend de inyeccion disponible; el texto queda solo en el portapapeles",
            );
            Ok(())
        }
    }
}

fn detect_backend() -> Backend {
    let has_wayland = has_display("WAYLAND_DISPLAY");
    let has_x11 = has_display("DISPLAY");

    debug::input_log(
        "backend",
        format!("presencia de displays; wayland={has_wayland}; x11={has_x11}"),
    );

    if has_wayland {
        Backend::Wayland
    } else if has_x11 {
        Backend::X11
    } else {
        Backend::ClipboardOnly
    }
}

fn has_display(name: &str) -> bool {
    let present = std::env::var_os(name).is_some_and(|value| !value.is_empty());
    debug::input_log("env", format!("variable {name} presente={present}"));
    present
}
