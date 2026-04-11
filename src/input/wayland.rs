use wrtype::WrtypeClient;

use crate::debug;

pub fn inject_text(text: &str) -> Result<(), String> {
    debug::input_log(
        "wayland",
        format!(
            "inicializando cliente Wayland para text={text:?}; chars={}",
            text.chars().count()
        ),
    );
    let mut client = WrtypeClient::new()
        .map_err(|err| format!("no se pudo inicializar el cliente Wayland: {err}"))?;

    debug::input_log(
        "wayland",
        "cliente Wayland inicializado; enviando type_text",
    );
    client
        .type_text(text)
        .map_err(|err| format!("no se pudo escribir el texto via Wayland: {err}"))?;
    debug::input_log("wayland", "type_text completo");
    Ok(())
}
