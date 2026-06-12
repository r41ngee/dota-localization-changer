use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_PATH: &str = "data/config.json";
const DEFAULT_LOGGER_LEVEL: &str = "INFO";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub dota_directory: Option<String>,
    #[serde(default = "default_logger")]
    pub logger_lvl: String,
}

fn default_logger() -> String {
    DEFAULT_LOGGER_LEVEL.to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dota_directory: None,
            logger_lvl: DEFAULT_LOGGER_LEVEL.to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = PathBuf::from(CONFIG_PATH);
        if let Ok(content) = std::fs::read_to_string(&path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let path = PathBuf::from(CONFIG_PATH);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, content);
        }
    }

    pub fn vpk_path(&self) -> Option<String> {
        self.dota_directory.as_ref().map(|dir| {
            format!("{}/game/dota_russian/pak01_dir.vpk", dir)
        })
    }
}
