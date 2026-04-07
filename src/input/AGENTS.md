# Waydot `src/input/` - Guía de Agentes

## Responsabilidad
Gestión de la inyección y copiado de texto (y en un futuro elementos multimedia) originado del panel al portapapeles y aplicación activa del sistema operativo, incluyendo manejo pragmático de backends (`wtype`, `xdotool` y modo "Fallback de portapapeles").

## Dependencias y Límites
- **Autonomía**: Toma la cadena de texto de entrada para copiarla y/o inyectarla dependiendo de las capacidades del display server actual (Wayland o X11).
- **Cero UI y Búsqueda**: No debe depender bajo ninguna circunstancia de `ui/` ni de `search/`.
- **Backends Seguros**: Abstenerse de implementar protocolos privilegiados o inestables de Wayland por defecto (e.g., `ext-virtual-keyboard` o `libei/reis`) sin documentar expresamente en `docs/decisions/`.
