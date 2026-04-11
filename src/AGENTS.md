# Waydot `src/` - Guia de Agentes

## Alcance

Este directorio contiene el codigo de producto de Waydot. La fuente de verdad estructural es `docs/architecture.md`; si un cambio altera responsabilidades entre modulos, actualiza esa documentacion y registra la decision en `docs/decisions/`.

## Responsabilidades por Modulo

### `main.rs`
Entry point minimo. Debe limitarse a construir la aplicacion y ejecutar `app.run()`.

### `app.rs`
Configura `adw::Application`, el ciclo de `startup`/`activate`, el `application_id`, el hold de background y la presentacion/reutilizacion de la ventana principal. No debe contener logica de UI ni reglas del historial del portapapeles.

### `config.rs`
Carga configuracion persistente local desde `dirs::data_dir()/waydot/config.json`. Actualmente define el atajo local de toggle, con `<Control><Shift>v` como default.

### `system.rs`
Bootstrap de integracion de escritorio. Asegura una entrada `.desktop` y un icono de usuario para `com.nothinc.waydot` mientras no exista empaquetado formal. No debe crecer hacia instaladores completos; cuando exista empaquetado, mover esa responsabilidad al paquete.

### `ui/`
Componentes GTK4/Libadwaita. Deben enfocarse en presentacion, composicion de widgets y delegacion de eventos. No colocar aqui persistencia, carga de datasets, seleccion de backends de inyeccion ni reglas de negocio del historial.

### `search/`
Motor de busqueda local para emojis, kaomojis y simbolos. Mantener aqui la logica de indexado y coincidencia; la UI solo debe consumir resultados.

### `data/`
Carga de datasets estaticos embebidos. Los errores de parseo pueden ser tratados como errores de empaquetado del binario cuando los datos vienen de `include_str!`.

### `clipboard/`
Historial y monitor de portapapeles. Mantener aqui deduplicacion, limites, pinning, persistencia y polling GDK. La notificacion a UI debe ser por callbacks/señales explícitas, no por acoplamiento directo con widgets.

### `input/`
Publicacion/copia del texto seleccionado. Mantiene el flujo activo de portapapeles y conserva modulos de insercion nativa no cableados al runtime. No mezclar con UI ni busqueda.

### `dbus/`
Activacion local, metodo `Toggle`, atajo local configurable y portales DBus. `GlobalShortcuts` sigue siendo roadmap; no documentar un atajo global real hasta implementar el portal correspondiente.

## Dependencias Permitidas

- `ui/` puede depender de `search/`, `clipboard/` e `input/` para delegar acciones.
- `app.rs` puede orquestar `ui`, `dbus`, `config` y `system`.
- `dbus/` puede depender de `app::APPLICATION_ID` para identidad de aplicacion.
- `clipboard/`, `search/`, `data/` e `input/` no deben depender de `ui/`.

## Reglas Locales

- No usar `unwrap()` en codigo de produccion salvo en errores de empaquetado claramente justificados.
- Ejecutar al menos `cargo check` para cambios de comportamiento cuando el entorno lo permita.
- Si se crea un nuevo subdirectorio bajo `src/`, crear tambien un `AGENTS.md` dentro de ese subdirectorio explicando su responsabilidad, dependencias y limites.
- Mantener configuracion de IA fuera de `src/`; debe vivir en `.ai/`.
- Mantener documentacion tecnica fuera de `src/`; debe vivir en `docs/`, excepto este archivo de guia local para agentes.
