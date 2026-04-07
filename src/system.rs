use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::app::APPLICATION_ID;

const DESKTOP_FILE: &str = "com.nothinc.waydot.desktop";
const ICON_FILE: &str = "com.nothinc.waydot.svg";
const APP_NAME: &str = "Waydot";
const APP_DESCRIPTION: &str = "Panel de emojis, simbolos y portapapeles";
const ICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="128" height="128" viewBox="0 0 128 128">
  <rect width="128" height="128" rx="28" fill="#9141ac"/>
  <circle cx="42" cy="46" r="9" fill="#ffffff"/>
  <circle cx="86" cy="46" r="9" fill="#ffffff"/>
  <path d="M38 78c12 16 40 16 52 0" fill="none" stroke="#ffffff" stroke-width="10" stroke-linecap="round"/>
  <path d="M29 104h70" fill="none" stroke="#ffffff" stroke-width="8" stroke-linecap="round" opacity=".7"/>
</svg>
"##;

pub fn ensure_user_desktop_integration() -> io::Result<()> {
    write_icon()?;
    write_desktop_file()
}

fn write_icon() -> io::Result<()> {
    let path = data_dir()
        .join("icons")
        .join("hicolor")
        .join("scalable")
        .join("apps")
        .join(ICON_FILE);

    write_if_changed(&path, ICON_SVG)
}

fn write_desktop_file() -> io::Result<()> {
    let exec_path = env::current_exe()?;
    let desktop_entry = format!(
        "[Desktop Entry]\nType=Application\nName={APP_NAME}\nComment={APP_DESCRIPTION}\nExec={}\nIcon={APPLICATION_ID}\nTerminal=false\nCategories=Utility;\nStartupNotify=true\n",
        quote_exec_path(&exec_path)
    );
    let path = data_dir().join("applications").join(DESKTOP_FILE);

    write_if_changed(&path, &desktop_entry)
}

fn write_if_changed(path: &Path, content: &str) -> io::Result<()> {
    if matches!(fs::read_to_string(path), Ok(existing) if existing == content) {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn quote_exec_path(path: &Path) -> String {
    let path = path.to_string_lossy();
    let escaped = path
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('`', "\\`");
    format!("\"{escaped}\"")
}

fn data_dir() -> PathBuf {
    dirs::data_dir().unwrap_or_else(|| PathBuf::from("."))
}
