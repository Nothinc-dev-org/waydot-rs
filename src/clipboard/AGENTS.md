# Waydot `src/clipboard/` - Guía de Agentes

## Responsabilidad
Gestión del historial y monitor de portapapeles. Mantiene la lógica de deduplicación, límites (ej. `MAX_ENTRIES`), anclaje (pinning), persistencia en el sistema local y polling asíncrono con GDK.

## Dependencias y Límites
- **Agnóstico a la UI**: No debe depender ni acoplarse directamente a `ui/`. Debe proveer los datos de estado para ser leídos mediante `Rc<RefCell<...>>` o comunicar cambios por callbacks/señales explícitas.
- **Sistema Local**: Utiliza serialización (ej. JSON) para persistir el historial en `dirs::data_dir()/waydot/clipboard_history.json`.
- **Restricciones Actuales**: El polling se realiza cada 500ms al soporte de texto plano. No se debe introducir complejidad mime-type u otros formatos binarios hasta que esté especificado en el Roadmap o en `docs/decisions/`.
