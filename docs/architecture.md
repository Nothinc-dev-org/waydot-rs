# Arquitectura de Waydot

## Vision General

Waydot es un panel flotante de entrada expresiva para escritorios Linux. Busca ofrecer acceso rapido a emojis, kaomojis, simbolos, GIFs y un historial del portapapeles, inspirado en el panel Win+. de Windows 11.

La arquitectura objetivo prioriza Wayland, seguridad de entrada y baja latencia. La implementacion actual es un MVP funcional centrado en datos locales, UI GTK4/Libadwaita, portapapeles de texto y estrategias pragmatistas de inyeccion.

## Estado Actual vs Roadmap

| Area | Implementado | Roadmap |
|------|--------------|---------|
| UI | Ventana `adw::Window`, `AdwHeaderBar`, `AdwViewStack`, `AdwViewSwitcher`, busqueda compartida para kaomojis/simbolos, subtabs internas en emojis, `gtk::GridView` para el catalogo completo y `gtk::FlowBox` para conjuntos pequenos | Composite templates para widgets complejos, adaptabilidad con breakpoints, posicionamiento avanzado |
| Emojis | Carga desde crate `emojis`, subtab `Todos` con busqueda local, subtab `Recientes` persistida en JSON | Anotaciones CLDR multilingues, variantes de tono/genero, categorias enriquecidas |
| Kaomojis | JSON embebido en `data/kaomojis.json` | Dataset ampliable por usuario |
| Simbolos | JSON embebido en `data/symbols.json` con keywords | Taxonomia mas completa y etiquetas personalizadas |
| GIFs | No implementado | Tenor API v2 con cache de miniaturas |
| Busqueda | Coincidencia por substring y subsecuencia | Indices persistentes, ranking, respuesta sub-10ms medible |
| Portapapeles | Texto plano via GDK polling cada 500ms, persistencia JSON | MIME types multiples, imagenes, HTML, SQLite/binario, daemon separado |
| Inyeccion | Copia al portapapeles + `wtype` o `xdotool` si existen | `zwp_virtual_keyboard_v1`, `libei/reis`, portalizacion segura |
| Activacion | Dos atajos por pestana (`<Control><Super>v` para clipboard, `<Control><Shift>period` para emojis), metodos DBus `Toggle`/`ShowClipboard`/`ShowEmojis`, activacion externa via `org.gtk.Actions.Activate`, ejecucion en background con `Application::hold`, icono de bandeja con ksni, identidad de escritorio de usuario y registro host en portal | XDG GlobalShortcuts Portal, empaquetado formal y servicio DBus robusto |

## Arquitectura de Alto Nivel

```text
+---------------------------------------------+
|     Activacion / DBus / Bandeja             |
|  show-clipboard + show-emojis + ksni tray   |
|  org.gtk.Actions.Activate (token Wayland)   |
+------------------+--------------------------+
                   | activa + selecciona tab
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

### 1. Aplicacion (`src/main.rs`, `src/app.rs`, `src/config.rs`, `src/system.rs`, `src/tray.rs`)

`main.rs` inicializa la aplicacion Libadwaita. `app.rs` construye `adw::Application` con application id `com.nothinc.waydot`, asegura metadata de escritorio de usuario durante el MVP, registra los atajos por pestana durante `startup`, lanza el icono de bandeja del sistema, mantiene vivo el proceso para background y crea o presenta la ventana durante `activate`.

`config.rs` carga `config.json` bajo `dirs::data_dir()/waydot/` y define dos aceleradores locales: `<Control><Super>v` para clipboard y `<Control><Shift>period` para emojis. `system.rs` genera una entrada `.desktop` y un icono de usuario para `com.nothinc.waydot` mientras el proyecto no tenga empaquetado formal. `tray.rs` ejecuta un servicio `ksni` (StatusNotifierItem) en un thread dedicado que muestra el icono de Waydot en la bandeja del sistema con menu para abrir la ventana o salir.

### 2. GUI (`src/ui/`)

La UI se construye programaticamente en Rust:

- `window.rs`: compone la ventana, la barra de busqueda compartida para kaomojis/simbolos, el `AdwViewStack` y el monitor de portapapeles.
- `emoji_grid.rs`: construye la pestana de emojis con subtabs `Recientes` y `Todos`, usa `gtk::GridView` para el catalogo completo de emojis y `gtk::FlowBox` para recientes, kaomojis y simbolos.
- `clipboard_panel.rs`: renderiza historial, acciones de copiar, eliminar, anclar y limpiar no anclados.

El estado compartido del historial usa `Rc<RefCell<ClipboardHistory>>` porque vive en el hilo principal de GTK. La busqueda compartida se dispara con `connect_search_changed` segun la pestana visible para kaomojis y simbolos, mientras que la subtab `Todos` de emojis tiene su propio `SearchEntry`.

### 3. Busqueda (`src/search/`)

`SearchEngine` carga tres colecciones:

- `EmojiEntry` desde `emojis::iter()`.
- `Kaomoji` desde JSON embebido.
- `Symbol` desde JSON embebido.

La funcion `fuzzy_match` primero revisa substring y luego coincidencia por subsecuencia ordenada. Este enfoque es simple y suficiente para el MVP, pero no implementa ranking, normalizacion avanzada, CLDR ni indices dedicados.

### 4. Datos (`src/data/`)

Los datasets de kaomojis y simbolos se embeben con `include_str!`, por lo que cambios en `data/*.json` requieren recompilar. Los errores de parseo usan `expect` porque son errores de empaquetado del binario, no datos de usuario en runtime.

### 4.5. Recientes de Emojis (`src/emoji_history.rs`)

Los emojis recientes se persisten en `emoji_history.json` bajo `dirs::data_dir()/waydot/`. La lista se mantiene deduplicada, ordenada del mas reciente al mas antiguo y limitada a un maximo fijo. La UI solo consume este estado para poblar la subtab `Recientes`.

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

### 7. DBus y Atajos (`src/dbus/`)

El MVP tiene tres piezas:

- `shortcuts.rs`: registra dos acciones (`app.show-clipboard` y `app.show-emojis`) con aceleradores configurables. Cada accion presenta la ventana y cambia a la pestana correspondiente. La funcion `show_tab` combina `ui::switch_to_tab` con `present_window` para abrir en la pestana correcta. La activacion externa usa `org.gtk.Actions.Activate` (interfaz estandar de GApplication) para obtener tokens de activacion de Wayland y traer la ventana a primer plano.
- `service.rs`: registra un objeto DBus en `/com/nothinc/Waydot` con interfaz `com.nothinc.Waydot` y metodos `Toggle`, `ShowClipboard` y `ShowEmojis`.
- `background.rs`: registra la app host en `org.freedesktop.host.portal.Registry`, solicita el portal `org.freedesktop.portal.Background` y publica un estado de background cuando esta disponible y permitido por el entorno.

La integracion objetivo con `org.freedesktop.portal.GlobalShortcuts` todavia no esta implementada. Cualquier cambio en activacion global debe documentarse como decision estructural en `docs/decisions/`.

## Flujo de Datos Principal

```text
Usuario presiona atajo global (Ctrl+Super+V o Ctrl+Shift+.)
  -> GNOME ejecuta gdbus call con org.gtk.Actions.Activate
  -> GApplication activa accion show-clipboard o show-emojis
  -> ui::switch_to_tab selecciona la pestana, window.present() trae a primer plano
  -> Si la pestana es Emojis, el usuario cambia entre subtabs Recientes/Todos
  -> Si el usuario busca en Todos, SearchEntry local filtra emojis via SearchEngine
  -> GridView actualiza su modelo visible sin materializar un widget por emoji del dataset completo
  -> Usuario selecciona emoji/kaomoji/simbolo
  -> Si es emoji, emoji_history guarda el glyph en recientes
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
