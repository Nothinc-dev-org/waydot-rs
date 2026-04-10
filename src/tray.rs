use ksni::{ToolTip, Tray, TrayService, menu::*};

use crate::app::APPLICATION_ID;

pub fn spawn() -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        let service = TrayService::new(WaydotTray);

        if let Err(e) = service.run() {
            eprintln!("Error en servicio de icono de bandeja: {:?}", e);
        }
    })
}

struct WaydotTray;

impl Tray for WaydotTray {
    fn id(&self) -> String {
        "waydot-tray".to_string()
    }

    fn icon_name(&self) -> String {
        APPLICATION_ID.to_string()
    }

    fn icon_theme_path(&self) -> String {
        dirs::home_dir()
            .map(|h| h.join(".local/share/icons").to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    fn title(&self) -> String {
        "Waydot".to_string()
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            icon_name: self.icon_name(),
            icon_pixmap: Vec::new(),
            title: "Waydot".to_string(),
            description: "Panel de emojis, símbolos y portapapeles".to_string(),
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            StandardItem {
                label: "Abrir Waydot".to_string(),
                activate: Box::new(|_| {
                    let _ = std::process::Command::new("gdbus")
                        .args([
                            "call",
                            "--session",
                            "--dest",
                            APPLICATION_ID,
                            "--object-path",
                            "/com/nothinc/waydot",
                            "--method",
                            "org.gtk.Actions.Activate",
                            "show-emojis",
                            "[]",
                            "{}",
                        ])
                        .spawn();
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Salir".to_string(),
                activate: Box::new(|_| {
                    std::process::exit(0);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}
