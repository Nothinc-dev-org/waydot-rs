use crate::data::{EmojiEntry, Kaomoji, Symbol, load_emojis, load_kaomojis, load_symbols};

#[derive(Clone)]
pub enum SearchResult {
    Emoji {
        glyph: String,
        name: String,
    },
    Kaomoji {
        text: String,
        category: String,
    },
    Symbol {
        char: String,
        name: String,
        category: String,
    },
}

impl SearchResult {
    pub fn display_text(&self) -> &str {
        match self {
            SearchResult::Emoji { glyph, .. } => glyph,
            SearchResult::Kaomoji { text, .. } => text,
            SearchResult::Symbol { char, .. } => char,
        }
    }

    pub fn label(&self) -> String {
        match self {
            SearchResult::Emoji { name, .. } => name.clone(),
            SearchResult::Kaomoji { category, .. } => category.clone(),
            SearchResult::Symbol { name, category, .. } => format!("{category} — {name}"),
        }
    }

    pub fn emoji_glyph(&self) -> Option<&str> {
        match self {
            SearchResult::Emoji { glyph, .. } => Some(glyph),
            _ => None,
        }
    }
}

pub struct SearchEngine {
    emojis: Vec<EmojiEntry>,
    kaomojis: Vec<Kaomoji>,
    symbols: Vec<Symbol>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            emojis: load_emojis(),
            kaomojis: load_kaomojis(),
            symbols: load_symbols(),
        }
    }

    pub fn search_emojis(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return self
                .emojis
                .iter()
                .map(|e| SearchResult::Emoji {
                    glyph: e.emoji.to_string(),
                    name: e.name.to_string(),
                })
                .collect();
        }

        let query_lower = query.to_lowercase();
        self.emojis
            .iter()
            .filter(|e| fuzzy_match(e.name, &query_lower))
            .map(|e| SearchResult::Emoji {
                glyph: e.emoji.to_string(),
                name: e.name.to_string(),
            })
            .collect()
    }

    pub fn search_kaomojis(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return self
                .kaomojis
                .iter()
                .map(|k| SearchResult::Kaomoji {
                    text: k.text.clone(),
                    category: k.category.clone(),
                })
                .collect();
        }

        let query_lower = query.to_lowercase();
        self.kaomojis
            .iter()
            .filter(|k| {
                fuzzy_match(&k.category, &query_lower) || fuzzy_match(&k.text, &query_lower)
            })
            .map(|k| SearchResult::Kaomoji {
                text: k.text.clone(),
                category: k.category.clone(),
            })
            .collect()
    }

    pub fn search_symbols(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return self
                .symbols
                .iter()
                .map(|s| SearchResult::Symbol {
                    char: s.char.clone(),
                    name: s.name.clone(),
                    category: s.category.clone(),
                })
                .collect();
        }

        let query_lower = query.to_lowercase();
        self.symbols
            .iter()
            .filter(|s| {
                fuzzy_match(&s.name, &query_lower)
                    || fuzzy_match(&s.char, &query_lower)
                    || s.keywords.iter().any(|kw| fuzzy_match(kw, &query_lower))
            })
            .map(|s| SearchResult::Symbol {
                char: s.char.clone(),
                name: s.name.clone(),
                category: s.category.clone(),
            })
            .collect()
    }
}

fn fuzzy_match(haystack: &str, needle: &str) -> bool {
    let haystack_lower = haystack.to_lowercase();
    if haystack_lower.contains(needle) {
        return true;
    }
    let mut haystack_chars = haystack_lower.chars();
    for needle_char in needle.chars() {
        loop {
            match haystack_chars.next() {
                Some(h) if h == needle_char => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}
