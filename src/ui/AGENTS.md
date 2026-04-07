# Waydot `src/ui/` - Guía de Agentes

## Responsabilidad
Módulo GTK4/Libadwaita para labores de rendering, estilo, lógica visual, ciclo de interfaces y manejo de eventos. Aquí viven todas las definiciones de los componentes principales de ventana, grids, campos de búsqueda y paneles.

## Dependencias y Límites
- **Integrador Permitido**: Este módulo sí tiene autorización para importar e invocar dependencias sobre `search/`, `clipboard/` e `input/`, actuando como el hilo conductor visible para el usuario.
- **Delegacy & Restricción**: No implementar persistencia a disco (le pertenece a `clipboard/`), descargas de red (cuando se habiliten APIs), inyección nativa de sistema o algoritmos base de filtrado directo. La UI es "tonta" respecto a datos concretos; invoca los módulos subyacentes y exhibe el Output.
