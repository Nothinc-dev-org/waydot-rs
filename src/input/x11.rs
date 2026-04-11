use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt as _;
use x11rb::protocol::xtest::ConnectionExt as _;
use x11rb::rust_connection::RustConnection;

use crate::debug;

const KEY_PRESS: u8 = 2;
const KEY_RELEASE: u8 = 3;
const XK_CONTROL_L: u32 = 0xffe3;
const XK_V_LOWER: u32 = 0x0076;
const XK_V_UPPER: u32 = 0x0056;

pub fn inject_paste() -> Result<(), String> {
    debug::input_log("x11", "abriendo conexion X11");
    let (connection, screen_num) =
        RustConnection::connect(None).map_err(|err| format!("no se pudo abrir X11: {err}"))?;
    debug::input_log(
        "x11",
        format!("conexion X11 abierta; screen_num={screen_num}"),
    );

    let control_keycode = find_keycode(&connection, XK_CONTROL_L)?;
    let v_keycode =
        find_keycode(&connection, XK_V_LOWER).or_else(|_| find_keycode(&connection, XK_V_UPPER))?;
    let root = connection.setup().roots[screen_num].root;
    debug::input_log(
        "x11",
        format!("keycodes resueltos; control={control_keycode}; v={v_keycode}; root={root}"),
    );

    send_key(&connection, KEY_PRESS, control_keycode, root)?;
    send_key(&connection, KEY_PRESS, v_keycode, root)?;
    send_key(&connection, KEY_RELEASE, v_keycode, root)?;
    send_key(&connection, KEY_RELEASE, control_keycode, root)?;
    connection
        .flush()
        .map_err(|err| format!("no se pudo vaciar la conexion X11: {err}"))?;
    debug::input_log("x11", "secuencia Ctrl+V enviada y flush completado");

    Ok(())
}

fn send_key(
    connection: &RustConnection,
    event_type: u8,
    keycode: u8,
    root: u32,
) -> Result<(), String> {
    debug::input_log(
        "x11",
        format!("enviando evento X11; event_type={event_type}; keycode={keycode}; root={root}"),
    );
    connection
        .xtest_fake_input(event_type, keycode, 0, root, 0, 0, 0)
        .map_err(|err| format!("no se pudo enviar el evento X11: {err}"))?;
    Ok(())
}

fn find_keycode(connection: &RustConnection, keysym: u32) -> Result<u8, String> {
    debug::input_log("x11", format!("buscando keycode para keysym=0x{keysym:x}"));
    let setup = connection.setup();
    let min_keycode = setup.min_keycode;
    let max_keycode = setup.max_keycode;
    let count = max_keycode - min_keycode + 1;
    let reply = connection
        .get_keyboard_mapping(min_keycode, count)
        .map_err(|err| format!("no se pudo leer el mapa del teclado X11: {err}"))?
        .reply()
        .map_err(|err| format!("no se pudo recibir el mapa del teclado X11: {err}"))?;

    for (index, chunk) in reply
        .keysyms
        .chunks(reply.keysyms_per_keycode as usize)
        .enumerate()
    {
        if chunk.contains(&keysym) {
            let keycode = min_keycode + index as u8;
            debug::input_log(
                "x11",
                format!("keysym=0x{keysym:x} resuelto a keycode={keycode}"),
            );
            return Ok(keycode);
        }
    }

    Err(format!(
        "no se encontro un keycode X11 para keysym 0x{keysym:x}"
    ))
}
