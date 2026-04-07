use gtk::gio;
use gtk::prelude::*;
use libadwaita as adw;

const OBJECT_PATH: &str = "/com/nothinc/Waydot";

pub fn activate_or_register(app: &adw::Application) {
    let Some(connection) = app.dbus_connection() else {
        return;
    };

    let interface_info = build_interface_info();
    let interface = interface_info
        .lookup_interface("com.nothinc.Waydot")
        .unwrap();

    let app_weak = app.downgrade();
    let _ = connection
        .register_object(OBJECT_PATH, &interface)
        .method_call(
            move |_conn, _sender, _path, _iface, method, _params, invocation| {
                if method == "Toggle" {
                    if let Some(app) = app_weak.upgrade() {
                        app.activate();
                    }
                }
                invocation.return_value(None);
            },
        )
        .build();
}

fn build_interface_info() -> gio::DBusNodeInfo {
    let xml = r#"
        <node>
            <interface name="com.nothinc.Waydot">
                <method name="Toggle"/>
            </interface>
        </node>
    "#;
    gio::DBusNodeInfo::for_xml(xml).expect("valid dbus interface xml")
}
