# 0004 - Subtabs de emojis, recientes persistidos y GridView

## Contexto

La pestana de emojis necesitaba separar dos experiencias distintas:

- `Recientes`, con acceso rapido a los ultimos emojis usados.
- `Todos`, con busqueda sobre el catalogo completo.

La primera implementacion de `Todos` usaba `gtk::FlowBox` con un widget por emoji. Eso funcionaba visualmente, pero introducia un retraso perceptible en el primer cambio de `Recientes` a `Todos`, porque GTK tenia que mapear y renderizar por primera vez un arbol grande de widgets.

## Decision

- Modelar la pestana `Emojis` como un contenedor con subtabs internas `Recientes` y `Todos`.
- Mantener la barra de busqueda solo dentro de `Todos`.
- Persistir los emojis recientes en `emoji_history.json` bajo `dirs::data_dir()/waydot/`.
- Usar `gtk::GridView` con `gio::ListStore` y `SignalListItemFactory` para el catalogo completo de emojis.
- Mantener `gtk::FlowBox` para conjuntos pequenos: recientes, kaomojis y simbolos.

## Consecuencias

- `Recientes` sobrevive entre sesiones y se mantiene deduplicado y acotado.
- El primer cambio a `Todos` deja de depender de la construccion de miles de widgets al mismo tiempo.
- La UI de emojis queda mas cercana al comportamiento esperado del producto: navegacion local con subtabs y busqueda contextual.
- El modulo `ui/` sigue siendo consumidor del estado; la persistencia de recientes vive fuera de la capa visual.
