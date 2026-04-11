# Waydot - Guia de Agentes

## Descripcion del Proyecto

Waydot es un panel de entrada expresiva para Linux que replica la experiencia de Win+. de Windows 11: selector de emojis, kaomojis, simbolos especiales e historial del portapapeles. El objetivo de producto tambien contempla GIFs via Tenor API, pero esa integracion todavia pertenece al roadmap.

El proyecto esta implementado en Rust con GTK4 y Libadwaita. La version actual ofrece UI GTK/Adw, busqueda sobre datasets locales, monitoreo de portapapeles de texto y publicacion de texto al portapapeles. La insercion nativa queda preservada como codigo desconectado en `src/input/`.

## Stack Tecnologico

- **Lenguaje**: Rust (edition 2024)
- **UI**: GTK4 0.9 + Libadwaita 0.7
- **Async runtime**: Tokio
- **HTTP**: Reqwest
- **Serialization**: Serde + serde_json
- **DBus / GIO**: zbus como dependencia y GIO DBus para activacion e integracion local
- **Datos locales**: crate `emojis` + JSON embebido en `data/`

## Convenciones Globales

### Nomenclatura
- Archivos y modulos: `snake_case`
- Tipos y structs: `PascalCase`
- Funciones y variables: `snake_case`
- Constantes: `SCREAMING_SNAKE_CASE`

### Estructura de Codigo
- Un modulo = una responsabilidad
- La logica de negocio NO va en los componentes de UI
- Los componentes de UI se limitan a presentacion y delegacion de eventos
- Los datos estaticos se cargan desde recursos embebidos o archivos JSON
- Mantener codigo en `src/`, documentacion en `docs/` y configuracion de IA en `.ai/`

### Patrones
- GTK4/Libadwaita para composicion de ventanas y widgets
- Signals y callbacks para comunicacion entre componentes
- Async/await con Tokio para operaciones de red y I/O cuando se implementen integraciones externas
- DBus/GIO para activacion y comunicacion inter-proceso
- Registrar decisiones estructurales en `docs/decisions/`

### Errores
- Usar `Result<T, E>` con tipos de error propios por modulo cuando sea necesario
- No usar `unwrap()` en codigo de produccion; permitido en tests y prototipos temporales justificados
- Propagar errores hacia arriba; manejarlos en el punto mas alto posible

### Tests
- Tests unitarios junto al codigo (`#[cfg(test)] mod tests`)
- Tests de integracion en `tests/`
- Para cambios de comportamiento, ejecutar al menos `cargo check` cuando el entorno lo permita

## Estructura de Directorios

```
src/
  main.rs           -- Entry point, setup de AdwApplication
  app.rs            -- Configuracion de aplicacion, ciclo de activacion y hold de background
  config.rs         -- Configuracion local persistente
  emoji_history.rs  -- Historial persistente de emojis recientes
  system.rs         -- Bootstrap de metadata de escritorio de usuario
  tray.rs           -- Icono de bandeja del sistema (ksni/StatusNotifierItem)
  ui/               -- Componentes de interfaz (ventana, grids, tabs, portapapeles)
  search/           -- Motor de busqueda local sobre emojis, kaomojis y simbolos
  clipboard/        -- Historial persistente y monitor de portapapeles de texto
  input/            -- Publicacion de texto al portapapeles y modulos de insercion nativa desconectados
  data/             -- Carga de datasets embebidos
  dbus/             -- Activacion, background portal y atajos por pestana
data/               -- Archivos de datos estaticos (JSON)
docs/               -- Documentacion tecnica
docs/decisions/     -- Decisiones estructurales y de arquitectura
.ai/                -- Configuracion y roles de IA
```

## Modulos y Responsabilidades

### `src/ui/`
Componentes de interfaz GTK4/Libadwaita. Actualmente construye una `adw::Window` con `AdwHeaderBar`, `AdwViewStack`, `AdwViewSwitcher`, una barra de busqueda compartida para kaomojis/simbolos, una pestana de emojis con subtabs `Recientes` y `Todos`, `gtk::GridView` para el catalogo completo de emojis, `gtk::FlowBox` para recientes/kaomojis/simbolos y panel de historial del portapapeles.

### `src/emoji_history.rs`
Persistencia local de emojis recientes. Guarda una lista deduplicada y limitada en `dirs::data_dir()/waydot/emoji_history.json` para poblar la subtab `Recientes`.

### `src/search/`
Motor de busqueda local con coincidencia simple/fuzzy por subsecuencia. Indexa emojis desde el crate `emojis`, kaomojis desde `data/kaomojis.json` y simbolos desde `data/symbols.json`.

### `src/clipboard/`
Gestion del historial de portapapeles de texto. El monitor usa polling con GTK/GDK cada 500ms, deduplica entradas, conserva anclados, limita entradas no ancladas y persiste en JSON bajo el directorio de datos del usuario.

### `src/tray.rs`
Icono de bandeja del sistema usando `ksni` (protocolo StatusNotifierItem). Ejecuta el servicio en un thread separado. El menu ofrece abrir la ventana (via `org.gtk.Actions.Activate`) y salir de la aplicacion.

### `src/config.rs`
Configuracion persistente local. Actualmente carga `config.json` bajo el directorio de datos del usuario (`dirs::data_dir()/waydot/`) y define dos atajos locales: `clipboard_shortcut` (`<Control><Super>v` por defecto) y `emoji_shortcut` (`<Control><Shift>period` por defecto).

### `src/system.rs`
Bootstrap de integracion de escritorio. Asegura una entrada `.desktop` y un icono de usuario para `com.nothinc.waydot` mientras no exista empaquetado formal. Cuando se agregue empaquetado, esta responsabilidad debe migrar a recursos instalados por el paquete.

### `src/input/`
Publicacion del texto/emoji seleccionado al portapapeles activo. El directorio conserva una implementacion modular de insercion nativa para Wayland y X11, pero no forma parte del runtime actual.

### `src/data/`
Carga, parseo y gestion de datasets. Emojis via crate `emojis`, kaomojis y simbolos desde archivos JSON embebidos con `include_str!`.

### `src/dbus/`
Activacion, background y atajos. Registra dos acciones (`app.show-clipboard` y `app.show-emojis`) con aceleradores configurables. Expone metodos DBus `Toggle`, `ShowClipboard` y `ShowEmojis` mediante GIO. La activacion externa usa `org.gtk.Actions.Activate` (interfaz estandar de GApplication) para obtener tokens de activacion de Wayland. Registra la app host con `org.freedesktop.host.portal.Registry` y solicita `org.freedesktop.portal.Background`. El portal `org.freedesktop.portal.GlobalShortcuts` queda como objetivo de integracion futura.
