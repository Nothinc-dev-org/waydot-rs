# 0002 - Background y atajo configurable del MVP

## Contexto

Waydot necesita mantenerse vivo aunque la ventana no este visible para poder seguir monitoreando el portapapeles. Tambien necesita una activacion rapida por teclado, con `Ctrl+Shift+V` como valor por defecto configurable.

GTK permite aceleradores de acciones dentro de la aplicacion, pero eso no equivale a un atajo global del sistema cuando otra app tiene el foco. La arquitectura objetivo ya reserva `org.freedesktop.portal.GlobalShortcuts` para una integracion posterior.

## Decision

- Mantener el proceso vivo con `Application::hold`.
- Ocultar la ventana al cerrar mediante `hide-on-close`, en vez de destruirla.
- Leer el acelerador de `config.json` bajo el directorio de datos de usuario, usando `<Control><Shift>v` como default.
- Asegurar una entrada `.desktop` y un icono de usuario para que `com.nothinc.waydot` tenga identidad de escritorio durante el MVP sin empaquetado.
- Registrar `com.nothinc.waydot` en `org.freedesktop.host.portal.Registry` antes de usar portales desde la app host.
- Solicitar el portal `org.freedesktop.portal.Background` y publicar un estado de background cuando el entorno lo permite.
- Mantener `GlobalShortcuts` fuera de este cambio para no mezclar una integracion de portal de sesion mas amplia con el cambio de ciclo de vida de la app.

## Consecuencias

- La app puede permanecer en segundo plano y reabrir la ventana rapidamente desde sus activaciones locales/DBus.
- El atajo es configurable, pero sigue siendo un acelerador GTK local hasta que se implemente `GlobalShortcuts`.
- La aparicion con icono en el listado de apps en segundo plano sigue dependiendo de que el entorno exponga un monitor de apps en background.
- En apps host/no sandboxed, algunos portales pueden rechazar `SetStatus`; Waydot conserva la solicitud de background y trata esa negativa como una limitacion esperada del entorno.
- Cuando exista empaquetado formal, la entrada `.desktop` y el icono deben moverse del bootstrap de usuario a los recursos instalados del paquete.
