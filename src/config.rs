use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "config.json";
pub const DEFAULT_CLIPBOARD_SHORTCUT: &str = "<Control><Super>v";
pub const DEFAULT_EMOJI_SHORTCUT: &str = "<Control><Shift>period";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_clipboard_shortcut")]
    pub clipboard_shortcut: String,
    #[serde(default = "default_emoji_shortcut")]
    pub emoji_shortcut: String,
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
            clipboard_shortcut: default_clipboard_shortcut(),
            emoji_shortcut: default_emoji_shortcut(),
        }
    }
}

fn default_clipboard_shortcut() -> String {
    DEFAULT_CLIPBOARD_SHORTCUT.to_string()
}

fn default_emoji_shortcut() -> String {
    DEFAULT_EMOJI_SHORTCUT.to_string()
}

fn config_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("waydot")
        .join(CONFIG_FILE)
}
