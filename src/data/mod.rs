mod kaomojis;
mod symbols;

pub use kaomojis::{Kaomoji, load_kaomojis};
pub use symbols::{Symbol, load_symbols};

pub struct EmojiEntry {
    pub emoji: &'static emojis::Emoji,
    pub name: &'static str,
}

pub fn load_emojis() -> Vec<EmojiEntry> {
    emojis::iter()
        .map(|e| EmojiEntry {
            emoji: e,
            name: e.name(),
        })
        .collect()
}
