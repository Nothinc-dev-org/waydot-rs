# Waydot

Panel de entrada expresiva para Linux: emojis, kaomojis, simbolos especiales e historial del portapapeles. Inspirado en el panel **Win+.** de Windows 11.

Construido con Rust, GTK4 y Libadwaita. Se ejecuta en segundo plano y aparece en la bandeja del sistema.

## Dependencias

### Fedora

```bash
sudo dnf install gtk4-devel libadwaita-devel dbus-devel
```

### Ubuntu / Debian

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev libdbus-1-dev
```

Tambien necesitas Rust (edicion 2024). Instalar con [rustup](https://rustup.rs/) si no lo tienes.

## Compilar y ejecutar

```bash
cargo build --release
./target/release/waydot
```

La primera ejecucion instala automaticamente el icono y la entrada `.desktop` en `~/.local/share/`.

## Atajos globales

Waydot se mantiene en segundo plano al cerrar la ventana. Para invocarlo desde cualquier aplicacion, configura atajos globales en GNOME que llamen a sus metodos DBus:

### Configurar via terminal

```bash
# Registrar los dos atajos personalizados
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings \
  "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/', \
    '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom1/']"

# Ctrl+Super+V -> Abrir en pestaña de Clipboard
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:\
/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ \
  name 'Waydot Clipboard'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:\
/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ \
  binding '<Control><Super>v'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:\
/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ \
  command "gdbus call --session --dest com.nothinc.waydot --object-path /com/nothinc/waydot --method org.gtk.Actions.Activate show-clipboard [] {}"

# Ctrl+Shift+. -> Abrir en pestaña de Emojis
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:\
/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom1/ \
  name 'Waydot Emojis'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:\
/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom1/ \
  binding '<Control><Shift>period'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:\
/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom1/ \
  command "gdbus call --session --dest com.nothinc.waydot --object-path /com/nothinc/waydot --method org.gtk.Actions.Activate show-emojis [] {}"
```

Tambien puedes configurarlos desde **Settings > Keyboard > Custom Shortcuts**.

> Si ya tienes atajos personalizados en GNOME, agrega las rutas nuevas al array existente en lugar de reemplazarlo.

### Interfaz DBus

Waydot expone tres metodos en la sesion DBus bajo `com.nothinc.Waydot`:

| Metodo | Descripcion |
|--------|-------------|
| `Toggle` | Muestra u oculta la ventana |
| `ShowClipboard` | Abre la ventana en la pestaña de clipboard |
| `ShowEmojis` | Abre la ventana en la pestaña de emojis |

Ejemplo de invocacion manual:

```bash
gdbus call --session \
  --dest com.nothinc.waydot \
  --object-path /com/nothinc/waydot \
  --method org.gtk.Actions.Activate \
  show-emojis "[]" "{}"
```

## Configuracion

Waydot guarda su configuracion en `~/.local/share/waydot/config.json`:

```json
{
  "clipboard_shortcut": "<Control><Super>v",
  "emoji_shortcut": "<Control><Shift>period"
}
```

Estos valores definen los aceleradores locales (cuando la ventana tiene foco). Para atajos globales, usa la configuracion de GNOME descrita arriba.

## Estructura del proyecto

```
src/
  main.rs          Punto de entrada
  app.rs           Ciclo de vida de la aplicacion
  config.rs        Configuracion persistente
  system.rs        Integracion de escritorio (.desktop, icono)
  tray.rs          Icono de bandeja del sistema (ksni)
  ui/              Ventana, grids, panel de portapapeles
  search/          Motor de busqueda local
  clipboard/       Historial y monitor de portapapeles
  input/           Inyeccion de texto (wtype, xdotool, fallback)
  data/            Datasets embebidos
  dbus/            Activacion, background y atajos
```

## Licencia

GPL-3.0
