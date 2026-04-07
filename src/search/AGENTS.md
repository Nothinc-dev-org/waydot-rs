# Waydot `src/search/` - Guía de Agentes

## Responsabilidad
Motor de búsqueda de lógica local. Agrupa las colecciones (Emojis provistos por el crate `emojis`, Kaomojis y Símbolos de JSON embebidos) y expone la capacidad de filtrado (fuzzy / substring matching) para cada subconjunto.

## Dependencias y Límites
- Recibe las dependencias desde `data/` o crates externos pertinentes (e.g., `emojis`), pero no tiene referencias lógicas ni dependencias hacia `ui/`.
- Las consultas son procesadas in-memory y su optimización actual se enfoca en respuestas sub-10ms sobre conjuntos finitos. Modificaciones complejas de índices (ej. SQLite) a futuro deben justificarse en `docs/decisions/`.
