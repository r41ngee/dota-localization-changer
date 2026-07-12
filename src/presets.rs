use crate::dotatypes::{Hero, Item};
use log::{debug, error, info, warn};
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
        if let Err(e) = std::fs::create_dir_all(&dir) {
            error!("Не удалось создать директорию пресетов {}: {}", dir.display(), e);
            return;
        }
        let file_path = dir.join(&self.filename);

        let data = PresetData {
            heroes: self.heroes.iter().map(|h| h.to_dict()).collect(),
            items: self.items.iter().map(|i| i.to_dict()).collect(),
        };

        match serde_json::to_string_pretty(&data) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&file_path, &content) {
                    error!("Не удалось записать пресет {}: {}", file_path.display(), e);
                } else {
                    debug!("Пресет записан: {} ({} байт)", file_path.display(), content.len());
                }
            }
            Err(e) => error!("Не удалось сериализовать пресет: {}", e),
        }
    }

    pub fn load(filename: &str) -> Self {
        let fname = if filename.ends_with(".json") {
            filename.to_string()
        } else {
            format!("{}.json", filename)
        };
        let file_path = PathBuf::from(PRESETS_DIR).join(&fname);
        let name = fname.replace(".json", "");

        match std::fs::read_to_string(&file_path) {
            Ok(content) => match serde_json::from_str::<PresetData>(&content) {
                Ok(data) => {
                    let heroes_count = data.heroes.len();
                    let items_count = data.items.len();
                    let heroes: Vec<Hero> = data
                        .heroes
                        .into_iter()
                        .filter_map(|v| {
                            serde_json::from_value(v).map_err(|e| {
                                warn!("Пропущен невалидный герой в пресете: {}", e);
                                e
                            })
                            .ok()
                        })
                        .collect();
                    let items: Vec<Item> = data
                        .items
                        .into_iter()
                        .filter_map(|v| {
                            serde_json::from_value(v).map_err(|e| {
                                warn!("Пропущен невалидный предмет в пресете: {}", e);
                                e
                            })
                            .ok()
                        })
                        .collect();
                    info!(
                        "Пресет '{}' загружен: {}/{} героев, {}/{} предметов",
                        name,
                        heroes.len(),
                        heroes_count,
                        items.len(),
                        items_count
                    );
                    Self {
                        filename: format!("{}.json", name),
                        heroes,
                        items,
                    }
                }
                Err(e) => {
                    error!("Не удалось распарсить пресет {}: {}", file_path.display(), e);
                    Self {
                        filename: format!("{}.json", name),
                        heroes: Vec::new(),
                        items: Vec::new(),
                    }
                }
            },
            Err(e) => {
                warn!("Не удалось прочитать пресет {}: {}", file_path.display(), e);
                Self {
                    filename: format!("{}.json", name),
                    heroes: Vec::new(),
                    items: Vec::new(),
                }
            }
        }
    }

    pub fn load_names() -> Vec<String> {
        let dir = PathBuf::from(PRESETS_DIR);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            error!("Не удалось создать директорию пресетов {}: {}", dir.display(), e);
            return Vec::new();
        }
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
        debug!("Найдено пресетов: {}", names.len());
        names
    }
}
