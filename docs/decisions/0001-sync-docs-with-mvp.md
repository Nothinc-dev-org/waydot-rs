# Decision 0001: Sincronizar documentacion con el MVP

## Estado

Aceptada.

## Contexto

La documentacion describia la arquitectura objetivo de Waydot como si varios componentes ya estuvieran implementados: Tenor API, XDG GlobalShortcuts Portal, `zwp_virtual_keyboard_v1`, `libei/reis`, persistencia SQLite/binaria, MIME types multiples y composite templates.

El codigo actual corresponde a un MVP local:

- UI GTK4/Libadwaita construida programaticamente.
- Busqueda local sobre `emojis` y JSON embebido.
- Historial de portapapeles de texto con polling GDK y persistencia JSON.
- Inyeccion mediante portapapeles, `wtype` o `xdotool`.
- Activacion local con accion `app.toggle` y metodo DBus `Toggle`.

## Decision

Separar explicitamente el estado implementado del roadmap en `docs/architecture.md` y actualizar `AGENTS.md` para que futuros agentes trabajen contra la estructura real del repositorio.

## Consecuencias

- La documentacion deja de prometer funcionalidad no implementada.
- Las metas de producto siguen preservadas como roadmap.
- Los cambios estructurales futuros deben documentarse en `docs/decisions/`.
- Si se agregan nuevos directorios bajo `src/`, se debe crear un `AGENTS.md` local para el modulo.
