use crate::dotatypes::{Hero, Item};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const PRESETS_DIR: &str = "presets";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetData {
    pub heroes: Vec<serde_json::Value>,
    pub items: Vec<serde_json::Value>,
}

pub struct Preset {
    pub filename: String,
    pub heroes: Vec<Hero>,
    pub items: Vec<Item>,
}

impl Preset {
    pub fn new(name: &str, heroes: Vec<Hero>, items: Vec<Item>) -> Self {
        Self {
            filename: format!("{}.json", name),
            heroes,
            items,
        }
    }

    pub fn save(&self) {
        let dir = PathBuf::from(PRESETS_DIR);
        let _ = std::fs::create_dir_all(&dir);
        let file_path = dir.join(&self.filename);

        let data = PresetData {
            heroes: self.heroes.iter().map(|h| h.to_dict()).collect(),
            items: self.items.iter().map(|i| i.to_dict()).collect(),
        };

        if let Ok(content) = serde_json::to_string_pretty(&data) {
            let _ = std::fs::write(&file_path, content);
        }
    }

    pub fn load(filename: &str) -> Self {
        let fname = if filename.ends_with(".json") { filename.to_string() } else { format!("{}.json", filename) };
        let file_path = PathBuf::from(PRESETS_DIR).join(&fname);
        let name = fname.replace(".json", "");

        if let Ok(content) = std::fs::read_to_string(&file_path) {
            if let Ok(data) = serde_json::from_str::<PresetData>(&content) {
                let heroes: Vec<Hero> = data
                    .heroes
                    .into_iter()
                    .filter_map(|v| serde_json::from_value(v).ok())
                    .collect();
                let items: Vec<Item> = data
                    .items
                    .into_iter()
                    .filter_map(|v| serde_json::from_value(v).ok())
                    .collect();
                return Self {
                    filename: format!("{}.json", name),
                    heroes,
                    items,
                };
            }
        }

        Self {
            filename: format!("{}.json", name),
            heroes: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn load_names() -> Vec<String> {
        let dir = PathBuf::from(PRESETS_DIR);
        let _ = std::fs::create_dir_all(&dir);
        let mut names = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "json").unwrap_or(false) {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        names.push(name.to_string());
                    }
                }
            }
        }
        names
    }

}
