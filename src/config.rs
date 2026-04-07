use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "config.json";
pub const DEFAULT_TOGGLE_SHORTCUT: &str = "<Control><Shift>v";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_toggle_shortcut")]
    pub toggle_shortcut: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let path = config_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str::<AppConfig>(&data) {
                return config;
            }
        }

        let config = Self::default();
        config.save(&path);
        config
    }

    fn save(&self, path: &PathBuf) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            toggle_shortcut: default_toggle_shortcut(),
        }
    }
}

fn default_toggle_shortcut() -> String {
    DEFAULT_TOGGLE_SHORTCUT.to_string()
}

fn config_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("waydot")
        .join(CONFIG_FILE)
}
