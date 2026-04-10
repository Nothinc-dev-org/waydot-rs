use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const HISTORY_FILE: &str = "emoji_history.json";
const MAX_ENTRIES: usize = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentEmojiHistory {
    glyphs: Vec<String>,
    #[serde(skip)]
    path: PathBuf,
}

impl RecentEmojiHistory {
    pub fn load() -> Self {
        let path = data_path();
        Self::load_from_path(path)
    }

    pub fn push(&mut self, glyph: &str) {
        if glyph.trim().is_empty() {
            return;
        }

        self.glyphs.retain(|entry| entry != glyph);
        self.glyphs.insert(0, glyph.to_string());
        self.glyphs.truncate(MAX_ENTRIES);
        self.save();
    }

    pub fn entries(&self) -> &[String] {
        &self.glyphs
    }

    fn load_from_path(path: PathBuf) -> Self {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(mut history) = serde_json::from_str::<RecentEmojiHistory>(&data) {
                history.path = path;
                history.glyphs.truncate(MAX_ENTRIES);
                return history;
            }
        }

        Self {
            glyphs: Vec::new(),
            path,
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

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::RecentEmojiHistory;

    #[test]
    fn deduplicates_and_keeps_latest_first() {
        let mut history = RecentEmojiHistory::load_from_path(unique_test_path());

        history.push("😀");
        history.push("🎉");
        history.push("😀");

        assert_eq!(history.entries(), &["😀".to_string(), "🎉".to_string()]);
    }

    #[test]
    fn limits_entries() {
        let mut history = RecentEmojiHistory::load_from_path(unique_test_path());

        for index in 0..30 {
            history.push(&format!("emoji-{index}"));
        }

        assert_eq!(history.entries().len(), 24);
        assert_eq!(
            history.entries().first().map(String::as_str),
            Some("emoji-29")
        );
        assert_eq!(
            history.entries().last().map(String::as_str),
            Some("emoji-6")
        );
    }

    fn unique_test_path() -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("waydot-emoji-history-{nanos}.json"))
    }
}
