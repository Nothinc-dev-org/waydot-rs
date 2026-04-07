# Waydot - Guia de Agentes

## Descripcion del Proyecto

Waydot es un panel de entrada expresiva para Linux que replica la experiencia de Win+. de Windows 11: selector de emojis, kaomojis, simbolos especiales e historial del portapapeles. El objetivo de producto tambien contempla GIFs via Tenor API, pero esa integracion todavia pertenece al roadmap.

El proyecto esta implementado en Rust con GTK4 y Libadwaita. La version actual es un MVP local: UI GTK/Adw, busqueda sobre datasets locales, monitoreo de portapapeles de texto y fallback de insercion mediante portapapeles + herramientas externas cuando estan disponibles.

## Stack Tecnologico

- **Lenguaje**: Rust (edition 2024)
- **UI**: GTK4 0.9 + Libadwaita 0.7
- **Async runtime**: Tokio
- **HTTP**: Reqwest
- **Serialization**: Serde + serde_json
- **DBus / GIO**: zbus como dependencia y GIO DBus para el MVP
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
  config.rs         -- Configuracion local persistente del MVP
  system.rs         -- Bootstrap de metadata de escritorio de usuario para el MVP
  ui/               -- Componentes de interfaz (ventana, grids, tabs, portapapeles)
  search/           -- Motor de busqueda local sobre emojis, kaomojis y simbolos
  clipboard/        -- Historial persistente y monitor de portapapeles de texto
  input/            -- Inyeccion/copia de texto via wtype, xdotool o clipboard fallback
  data/             -- Carga de datasets embebidos
  dbus/             -- Activacion local, background portal y accion de atajo del MVP
data/               -- Archivos de datos estaticos (JSON)
docs/               -- Documentacion tecnica
docs/decisions/     -- Decisiones estructurales y de arquitectura
.ai/                -- Configuracion y roles de IA
```

## Modulos y Responsabilidades

### `src/ui/`
Componentes de interfaz GTK4/Libadwaita. Actualmente construye una `adw::Window` con `AdwHeaderBar`, `AdwViewStack`, `AdwViewSwitcher`, `gtk::SearchEntry`, grids con `gtk::FlowBox` y panel de historial del portapapeles.

### `src/search/`
Motor de busqueda local con coincidencia simple/fuzzy por subsecuencia. Indexa emojis desde el crate `emojis`, kaomojis desde `data/kaomojis.json` y simbolos desde `data/symbols.json`.

### `src/clipboard/`
Gestion del historial de portapapeles de texto. El monitor usa polling con GTK/GDK cada 500ms, deduplica entradas, conserva anclados, limita entradas no ancladas y persiste en JSON bajo el directorio de datos del usuario.

### `src/config.rs`
Configuracion persistente local del MVP. Actualmente carga `config.json` bajo el directorio de datos del usuario (`dirs::data_dir()/waydot/`) y define el atajo local de toggle, con `<Control><Shift>v` como valor por defecto.

### `src/system.rs`
Bootstrap de integracion de escritorio del MVP. Asegura una entrada `.desktop` y un icono de usuario para `com.nothinc.waydot` mientras no exista empaquetado formal. Cuando se agregue empaquetado, esta responsabilidad debe migrar a recursos instalados por el paquete.

### `src/input/`
Insercion del texto/emoji seleccionado en la aplicacion activa. La implementacion actual copia al portapapeles y luego intenta:
1. `wtype` en Wayland si existe.
2. `xdotool` si existe.
3. Clipboard fallback sin pegado automatico.

La arquitectura objetivo sigue contemplando `zwp_virtual_keyboard_v1` y `libei/reis`, pero no estan implementados.

### `src/data/`
Carga, parseo y gestion de datasets. Emojis via crate `emojis`, kaomojis y simbolos desde archivos JSON embebidos con `include_str!`.

### `src/dbus/`
Activacion, background y atajo del MVP. Actualmente registra una accion `app.toggle` con acelerador configurable (`<Control><Shift>v` por defecto), expone un metodo DBus local `Toggle` mediante GIO, registra la app host con `org.freedesktop.host.portal.Registry` y solicita `org.freedesktop.portal.Background` cuando esta disponible. El portal `org.freedesktop.portal.GlobalShortcuts` queda como objetivo de integracion futura para atajos globales reales.
