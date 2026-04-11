# Waydot `src/ui/` - Guía de Agentes

## Responsabilidad
Módulo GTK4/Libadwaita para labores de rendering, estilo, lógica visual, ciclo de interfaces y manejo de eventos. Aquí viven todas las definiciones de los componentes principales de ventana, grids, campos de búsqueda y paneles.

## Dependencias y Límites
- **Integrador Permitido**: Este módulo sí tiene autorización para importar e invocar dependencias sobre `search/`, `clipboard/` e `input/`, actuando como el hilo conductor visible para el usuario.
- **Persistencia Externa**: La subtab de emojis recientes consume estado persistido desde `emoji_history.rs`; `ui/` solo lo presenta y reacciona a eventos.
- **Widgets Actuales**: Para datasets grandes de emojis, preferir `gtk::GridView`/modelos en vez de materializar miles de hijos en `gtk::FlowBox`. `FlowBox` sigue siendo válido para conjuntos pequeños como recientes, kaomojis y símbolos.
- **Texto Derivado del Usuario**: Cualquier preview o label construida desde contenido del portapapeles debe normalizar saltos de linea y truncarse por caracteres Unicode, nunca por indices de bytes, para evitar panics al recibir box-drawing, emojis u otros caracteres multibyte.
- **Delegacy & Restricción**: No implementar persistencia a disco dentro del módulo UI, descargas de red (cuando se habiliten APIs), inyección nativa de sistema o algoritmos base de filtrado directo. La UI es "tonta" respecto a datos concretos; invoca los módulos subyacentes y exhibe el Output.
