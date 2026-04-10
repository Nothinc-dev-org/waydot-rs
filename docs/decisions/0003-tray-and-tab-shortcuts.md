# 0003 - Bandeja del sistema y atajos por pestana

## Contexto

Waydot necesita presencia visual en la bandeja del sistema cuando la ventana esta oculta, y atajos globales que abran directamente en una pestana especifica (clipboard o emojis) en lugar de un toggle generico.

En Wayland, las aplicaciones no pueden traer su ventana a primer plano sin un token de activacion del compositor. Los metodos DBus personalizados no reciben ese token, pero la interfaz estandar `org.gtk.Actions.Activate` de GApplication si lo gestiona correctamente.

## Decision

- Agregar un icono de bandeja del sistema usando `ksni` (protocolo StatusNotifierItem), ejecutado en un thread dedicado desde `connect_startup`.
- Reemplazar el atajo unico `toggle_shortcut` por dos atajos especificos: `clipboard_shortcut` (`<Control><Super>v`) y `emoji_shortcut` (`<Control><Shift>period`).
- Exponer el `ViewStack` mediante un thread-local en `ui/window.rs` con una funcion `switch_to_tab` para cambiar pestana desde fuera del modulo UI.
- Registrar dos acciones GApplication (`show-clipboard` y `show-emojis`) que presentan la ventana en la pestana correspondiente.
- Agregar metodos DBus `ShowClipboard` y `ShowEmojis` en la interfaz `com.nothinc.Waydot`.
- Usar `org.gtk.Actions.Activate` (ruta `/com/nothinc/waydot`) para la activacion externa desde atajos globales de GNOME, en lugar de los metodos DBus personalizados, para obtener el token de activacion de Wayland.
- Configurar los atajos globales de GNOME via `gsettings` para custom-keybindings del settings-daemon.

## Consecuencias

- El icono de bandeja da retroalimentacion visual de que Waydot esta activo en segundo plano.
- Los atajos globales abren la ventana directamente en la pestana deseada y la traen a primer plano correctamente en Wayland.
- `Super+V` no se puede usar como atajo global porque GNOME captura la tecla Super internamente; se usa `Ctrl+Super+V` como alternativa.
- Los atajos globales dependen de la configuracion de GNOME (gsettings), no de la app. Si el usuario usa otro DE, debe configurar atajos equivalentes que invoquen las acciones via `org.gtk.Actions.Activate`.
- Los metodos DBus personalizados (`ShowClipboard`, `ShowEmojis`) siguen disponibles para uso programatico, pero no traen la ventana a primer plano en Wayland; para eso se debe usar la ruta de GApplication.
- El thread-local para el ViewStack acopla la logica de cambio de pestana al hilo principal de GTK, lo cual es correcto dado que GTK es single-threaded.
