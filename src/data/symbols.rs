use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SymbolEntry {
    char: String,
    name: String,
    keywords: Vec<String>,
}

#[derive(Debug)]
pub struct Symbol {
    pub char: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct SymbolCategory {
    pub name: String,
    entries: Vec<SymbolEntry>,
}

#[derive(Deserialize)]
struct SymbolFile {
    categories: Vec<SymbolCategory>,
}

pub fn load_symbols() -> Vec<Symbol> {
    let raw = include_str!("../../data/symbols.json");
    let file: SymbolFile = serde_json::from_str(raw).expect("invalid symbols.json");
    file.categories
        .into_iter()
        .flat_map(|cat| {
            let name = cat.name;
            cat.entries.into_iter().map(move |e| Symbol {
                char: e.char,
                name: e.name,
                keywords: e.keywords,
                category: name.clone(),
            })
        })
        .collect()
}
