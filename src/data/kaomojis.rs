use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KaomojiCategory {
    pub name: String,
    pub entries: Vec<String>,
}

#[derive(Debug)]
pub struct Kaomoji {
    pub text: String,
    pub category: String,
}

#[derive(Deserialize)]
struct KaomojiFile {
    categories: Vec<KaomojiCategory>,
}

pub fn load_kaomojis() -> Vec<Kaomoji> {
    let raw = include_str!("../../data/kaomojis.json");
    let file: KaomojiFile = serde_json::from_str(raw).expect("invalid kaomojis.json");
    file.categories
        .into_iter()
        .flat_map(|cat| {
            let name = cat.name;
            cat.entries.into_iter().map(move |text| Kaomoji {
                text,
                category: name.clone(),
            })
        })
        .collect()
}
