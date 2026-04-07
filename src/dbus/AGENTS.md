# Waydot `src/dbus/` - Guía de Agentes

## Responsabilidad
Manejar la activación local, la inscripción del método D-Bus `Toggle`, así cómo la configuración del atajo local ("shortcut") configurable y la integración con portales D-Bus (ej. `org.freedesktop.portal.Background`).

## Dependencias y Límites
- Puede depender de la constante de Identidad de Aplicación (`app::APPLICATION_ID`).
- **Separación Lógica**: No se mete en la lógica directa de visualización o los contenidos cargados; solo maneja eventos, registros DBus e inputs de atajos globales para gatillar `Toggle` u otras funciones locales.
- **Roadmap Constraint**: No asumas, no implementes y no documentes el comportamiento completo del portal de `GlobalShortcuts` como "estable", ya que sigue siendo un objetivo a futuro, a menos que un archivo en `docs/decisions/` lo habilite explícitamente.
