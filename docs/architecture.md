# Arquitectura de Waydot

## Vision General

Waydot es un panel flotante de entrada expresiva para escritorios Linux. Busca ofrecer acceso rapido a emojis, kaomojis, simbolos, GIFs y un historial del portapapeles, inspirado en el panel Win+. de Windows 11.

La arquitectura objetivo prioriza Wayland, seguridad de entrada y baja latencia. La implementacion actual es un MVP funcional centrado en datos locales, UI GTK4/Libadwaita, portapapeles de texto y estrategias pragmatistas de inyeccion.

## Estado Actual vs Roadmap

| Area | Implementado | Roadmap |
|------|--------------|---------|
| UI | Ventana `adw::Window`, `AdwHeaderBar`, `AdwViewStack`, `AdwViewSwitcher`, `gtk::SearchEntry`, `gtk::FlowBox` | Composite templates para widgets complejos, adaptabilidad con breakpoints, posicionamiento avanzado |
| Emojis | Carga desde crate `emojis` y busqueda por nombre | Anotaciones CLDR multilingues, variantes de tono/genero, categorias enriquecidas |
| Kaomojis | JSON embebido en `data/kaomojis.json` | Dataset ampliable por usuario |
| Simbolos | JSON embebido en `data/symbols.json` con keywords | Taxonomia mas completa y etiquetas personalizadas |
| GIFs | No implementado | Tenor API v2 con cache de miniaturas |
| Busqueda | Coincidencia por substring y subsecuencia | Indices persistentes, ranking, respuesta sub-10ms medible |
| Portapapeles | Texto plano via GDK polling cada 500ms, persistencia JSON | MIME types multiples, imagenes, HTML, SQLite/binario, daemon separado |
| Inyeccion | Copia al portapapeles + `wtype` o `xdotool` si existen | `zwp_virtual_keyboard_v1`, `libei/reis`, portalizacion segura |
| Activacion | Accion local configurable (`<Control><Shift>v` por defecto), metodo DBus `Toggle` con GIO, ejecucion en background con `Application::hold`, identidad de escritorio de usuario y registro host en portal | XDG GlobalShortcuts Portal, empaquetado formal y servicio DBus robusto |

## Arquitectura de Alto Nivel

```text
+---------------------------------------------+
|        Activacion local / DBus MVP          |
|        app.toggle + com.nothinc.Waydot      |
+------------------+--------------------------+
                   | activa
                   v
+---------------------------------------------+
|            Waydot GUI (GTK4/Adw)            |
|  +----------+----------+----------+-------+ |
|  |  Emojis  | Kaomojis | Simbolos | Clip- | |
|  |          |          |          | board | |
|  +----+-----+----+-----+----+-----+---+---+ |
|       |          |          |         |     |
|  +----v----------v----------v---------v---+ |
|  |       Motor de Busqueda Local          | |
|  |   emojis crate + JSON embebido         | |
|  +----------------------------------------+ |
+------------------+--------------------------+
                   | seleccion
                   v
+---------------------------------------------+
|              Inyeccion MVP                  |
|       clipboard + wtype/xdotool/fallback    |
+---------------------------------------------+
```

## Componentes Principales

### 1. Aplicacion (`src/main.rs`, `src/app.rs`, `src/config.rs`, `src/system.rs`)

`main.rs` inicializa la aplicacion Libadwaita. `app.rs` construye `adw::Application` con application id `com.nothinc.waydot`, asegura metadata de escritorio de usuario durante el MVP, registra la accion de atajo durante `startup`, mantiene vivo el proceso para background y crea o presenta la ventana durante `activate`.

`config.rs` carga `config.json` bajo `dirs::data_dir()/waydot/` y define `<Control><Shift>v` como acelerador local por defecto. `system.rs` genera una entrada `.desktop` y un icono de usuario para `com.nothinc.waydot` mientras el proyecto no tenga empaquetado formal.

### 2. GUI (`src/ui/`)

La UI se construye programaticamente en Rust:

- `window.rs`: compone la ventana, el buscador, el `AdwViewStack` y el monitor de portapapeles.
- `emoji_grid.rs`: construye paginas y resultados para emojis, kaomojis y simbolos con `gtk::FlowBox`.
- `clipboard_panel.rs`: renderiza historial, acciones de copiar, eliminar, anclar y limpiar no anclados.

El estado compartido del historial usa `Rc<RefCell<ClipboardHistory>>` porque vive en el hilo principal de GTK. La busqueda se dispara con `connect_search_changed` segun la pestana visible.

### 3. Busqueda (`src/search/`)

`SearchEngine` carga tres colecciones:

- `EmojiEntry` desde `emojis::iter()`.
- `Kaomoji` desde JSON embebido.
- `Symbol` desde JSON embebido.

La funcion `fuzzy_match` primero revisa substring y luego coincidencia por subsecuencia ordenada. Este enfoque es simple y suficiente para el MVP, pero no implementa ranking, normalizacion avanzada, CLDR ni indices dedicados.

### 4. Datos (`src/data/`)

Los datasets de kaomojis y simbolos se embeben con `include_str!`, por lo que cambios en `data/*.json` requieren recompilar. Los errores de parseo usan `expect` porque son errores de empaquetado del binario, no datos de usuario en runtime.

### 5. Portapapeles (`src/clipboard/`)

El historial actual soporta texto plano:

- `ClipboardMonitor` consulta `gdk::Display::default().clipboard().read_text_async()` cada 500ms.
- `ClipboardHistory` ignora entradas vacias, deduplica contenido existente, mantiene timestamps, soporta anclado y persiste automaticamente.
- El panel de clipboard se refresca cuando el monitor agrega una entrada nueva y cuando las acciones de limpiar, anclar o eliminar cambian el historial.
- El archivo se guarda como `clipboard_history.json` dentro de `dirs::data_dir()/waydot/`.
- `MAX_ENTRIES` limita 100 entradas no ancladas; las entradas ancladas se preservan sobre ese limite.

No hay soporte actual para HTML, imagenes, MIME types multiples ni un daemon separado.

### 6. Inyeccion (`src/input/`)

`inject_text` siempre copia el texto al portapapeles de GTK/GDK. Despues intenta elegir un backend:

| Backend | Condicion | Accion |
|---------|-----------|--------|
| `wtype` | `WAYLAND_DISPLAY` existe y `wtype` esta instalado | Ejecuta `wtype -- <texto>` |
| `xdotool` | `xdotool` esta instalado | Ejecuta `xdotool key --clearmodifiers ctrl+v` |
| ClipboardOnly | Ninguno disponible | Deja el texto copiado |

Este diseno evita depender desde el inicio de protocolos privilegiados de Wayland, pero tiene limitaciones: `wtype` no es universal, `xdotool` aplica principalmente a X11 y el fallback no pega automaticamente.

### 7. DBus y Atajo (`src/dbus/`)

El MVP tiene tres piezas:

- `shortcuts.rs`: registra una accion `app.toggle` con acelerador configurable (`<Control><Shift>v` por defecto).
- `service.rs`: registra un objeto DBus en `/com/nothinc/Waydot` con interfaz `com.nothinc.Waydot` y metodo `Toggle`.
- `background.rs`: registra la app host en `org.freedesktop.host.portal.Registry`, solicita el portal `org.freedesktop.portal.Background` y publica un estado de background cuando esta disponible y permitido por el entorno.

La integracion objetivo con `org.freedesktop.portal.GlobalShortcuts` todavia no esta implementada. Cualquier cambio en activacion global debe documentarse como decision estructural en `docs/decisions/`.

## Flujo de Datos Principal

```text
Usuario abre Waydot
  -> AdwApplication activa o presenta ventana existente
  -> Usuario escribe termino de busqueda
  -> SearchEngine filtra el dataset de la pestana visible
  -> UI reemplaza el contenido del ScrolledWindow con nuevos botones
  -> Usuario selecciona emoji/kaomoji/simbolo
  -> input::inject_text copia y usa wtype/xdotool/fallback
```

Flujo de portapapeles:

```text
ClipboardMonitor tick cada 500ms
  -> Lee texto del portapapeles via GDK
  -> Si cambio y no esta vacio, ClipboardHistory::push
  -> Deduplica, limita no anclados y guarda JSON
  -> Notifica a la UI
  -> Panel refresca si la pestana Clipboard esta visible
```

## Decisiones Tecnicas

### Por que Rust

- Seguridad de memoria para manejar datos del sistema.
- Rendimiento nativo para mantener la UI reactiva.
- Ecosistema maduro para GTK4/Libadwaita, Serde y DBus.

### Por que GTK4/Libadwaita

- Integracion nativa con GNOME y Wayland.
- Widgets de navegacion como `AdwViewStack` y `AdwViewSwitcher`.
- Buen encaje con una herramienta de escritorio pequena y persistente.

### Por que un MVP pragmatista de entrada

Wayland restringe por diseno la inyeccion global de entrada. Por eso la implementacion actual usa portapapeles + herramientas externas opcionales, mientras se reserva la arquitectura objetivo para `zwp_virtual_keyboard_v1`, `libei/reis` o portales cuando se defina una integracion segura.

## Reglas de Evolucion

- Mantener codigo de producto en `src/`.
- Mantener documentacion tecnica en `docs/`.
- Mantener roles y configuracion de IA en `.ai/`.
- Registrar decisiones estructurales en `docs/decisions/`.
- Si se crea un nuevo directorio bajo `src/`, agregar un `AGENTS.md` local que explique responsabilidad, dependencias y limites del modulo.
- Evitar que la documentacion afirme como implementadas funcionalidades que solo estan en roadmap.
