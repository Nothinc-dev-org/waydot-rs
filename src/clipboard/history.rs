use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const MAX_ENTRIES: usize = 100;
const HISTORY_FILE: &str = "clipboard_history.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub pinned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardHistory {
    entries: Vec<ClipboardEntry>,
    #[serde(skip)]
    path: PathBuf,
}

impl ClipboardHistory {
    pub fn load() -> Self {
        let path = data_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(mut history) = serde_json::from_str::<ClipboardHistory>(&data) {
                history.path = path;
                return history;
            }
        }
        Self {
            entries: Vec::new(),
            path,
        }
    }

    pub fn push(&mut self, content: String) {
        if content.trim().is_empty() {
            return;
        }
        if let Some(pos) = self.entries.iter().position(|e| e.content == content) {
            let mut entry = self.entries.remove(pos);
            entry.timestamp = Utc::now();
            self.entries.insert(0, entry);
        } else {
            self.entries.insert(
                0,
                ClipboardEntry {
                    content,
                    timestamp: Utc::now(),
                    pinned: false,
                },
            );
        }
        self.trim();
        self.save();
    }

    pub fn entries(&self) -> &[ClipboardEntry] {
        &self.entries
    }

    pub fn toggle_pin(&mut self, index: usize) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.pinned = !entry.pinned;
            self.save();
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.entries.len() {
            self.entries.remove(index);
            self.save();
        }
    }

    pub fn clear_unpinned(&mut self) {
        self.entries.retain(|e| e.pinned);
        self.save();
    }

    fn trim(&mut self) {
        let pinned_count = self.entries.iter().filter(|e| e.pinned).count();
        while self.entries.len() > MAX_ENTRIES + pinned_count {
            if let Some(pos) = self.entries.iter().rposition(|e| !e.pinned) {
                self.entries.remove(pos);
            } else {
                break;
            }
        }
    }

    fn save(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(&self.path, data);
        }
    }
}

fn data_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("waydot")
        .join(HISTORY_FILE)
}
