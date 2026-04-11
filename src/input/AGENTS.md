# Waydot `src/input/` - Guía de Agentes

## Responsabilidad
Gestión de la publicación y copiado de texto (y en un futuro elementos multimedia) originado del panel al portapapeles del sistema. Este módulo también puede conservar implementaciones de inserción nativa desacopladas del runtime cuando estén en investigación.

## Dependencias y Límites
- **Autonomía**: Toma la cadena de texto de entrada para copiarla al portapapeles y, cuando se documente explícitamente, puede albergar rutas de inserción nativa en módulos separados.
- **Cero UI y Búsqueda**: No debe depender bajo ninguna circunstancia de `ui/` ni de `search/`.
- **Runtime Honesto**: El flujo conectado a la UI solo debe exponer capacidades que funcionen de forma verificable en el entorno objetivo.
- **Codigo Estacionado**: Si una ruta nativa no es portable todavia, modularizarla y dejarla desacoplada del runtime en lugar de ocultar la limitacion con heuristicas.
