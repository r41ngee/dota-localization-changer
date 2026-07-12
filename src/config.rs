use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const APP_NAME: &str = "dota-localization-changer";
const CONFIG_FILENAME: &str = "config.json";
const LOG_FILENAME: &str = "app.log";
const DEFAULT_LOGGER_LEVEL: &str = "INFO";

fn app_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_NAME)
}

fn config_path() -> PathBuf {
    app_dir().join(CONFIG_FILENAME)
}

pub fn log_path() -> PathBuf {
    app_dir().join("logs").join(LOG_FILENAME)
}

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
        let path = config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
                log::warn!("Не удалось распарсить {}: {}. Используются значения по умолчанию", path.display(), e);
                Self::default()
            }),
            Err(e) => {
                log::warn!("Конфиг не найден ({}): {}. Используются значения по умолчанию", path.display(), e);
                Self::default()
            }
        }
    }

    pub fn save(&self) {
        let path = config_path();
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!("Не удалось создать директорию {}: {}", parent.display(), e);
                return;
            }
        }
        match serde_json::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&path, content) {
                    log::error!("Не удалось записать {}: {}", path.display(), e);
                }
            }
            Err(e) => log::error!("Не удалось сериализовать конфиг: {}", e),
        }
    }

    pub fn vpk_path(&self) -> Option<String> {
        self.dota_directory.as_ref().map(|dir| {
            format!("{}/game/dota_russian/pak01_dir.vpk", dir)
        })
    }
}
