# Waydot `src/data/` - Guía de Agentes

## Responsabilidad
Responsable de mantener y cargar los datasets estáticos embebidos de Waydot (e.g., Kaomojis, Símbolos).

## Dependencias y Límites
- **Embebido**: Actualmente todos los archivos de datos (e.g. `data/kaomojis.json`, `data/symbols.json`) son embebidos dentro del binario usando `include_str!`. Recompilación es requerida después de modificar los datos brutos.
- **Sin UI**: Este módulo no tiene conocimiento sobre estado gráfico ni UI, no depende de `ui/`.
- **Manejo de Errores**: Como estos datos son compilados directamente, pueden usarse aserciones duras (`unwrap`/`expect`) para manejar problemas de parseo (ya que implican binarios defectuosos empacados y no estado corrupto en runtime).
