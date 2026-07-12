use log::{debug, error, info, warn};
use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::OnceLock;

mod config;
mod dotatypes;
mod kvparser;
mod presets;
mod vpk;

struct AppData {
    heroes: Vec<dotatypes::Hero>,
    items: Vec<dotatypes::Item>,
    filtered_heroes: Vec<usize>,
    filtered_items: Vec<usize>,
    config: config::Config,
    filter_hero_query: String,
    filter_item_query: String,
    status_message: String,
}

fn load_json<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse {}: {}", path, e))
}

fn app_data() -> &'static Mutex<AppData> {
    static DATA: OnceLock<Mutex<AppData>> = OnceLock::new();
    DATA.get_or_init(|| {
        let cfg = config::Config::load();
        info!("Конфиг загружен: level={}", cfg.logger_lvl);

        let heroes = load_json::<Vec<dotatypes::Hero>>("data/hero_tags.json")
            .unwrap_or_else(|e| {
                warn!("{}", e);
                Vec::new()
            });
        info!("Загружено героев: {}", heroes.len());

        let items = load_json::<Vec<dotatypes::Item>>("data/items_tags.json")
            .unwrap_or_else(|e| {
                warn!("{}", e);
                Vec::new()
            });
        info!("Загружено предметов: {}", items.len());

        let h_idx: Vec<usize> = (0..heroes.len()).collect();
        let i_idx: Vec<usize> = (0..items.len()).collect();
        Mutex::new(AppData {
            heroes,
            items,
            filtered_heroes: h_idx,
            filtered_items: i_idx,
            config: cfg,
            filter_hero_query: String::new(),
            filter_item_query: String::new(),
            status_message: String::new(),
        })
    })
}

fn set_status(msg: &str) {
    if let Ok(mut d) = app_data().lock() {
        d.status_message = msg.to_string();
    } else {
        error!("Mutex poisoned при установке статуса");
    }
}

fn hero_json(d: &AppData) -> String {
    let sub: Vec<&dotatypes::Hero> = d.filtered_heroes.iter().map(|&i| &d.heroes[i]).collect();
    serde_json::to_string(&sub).unwrap_or_else(|e| {
        error!("Ошибка сериализации героев: {}", e);
        String::new()
    })
}

fn item_json(d: &AppData) -> String {
    let sub: Vec<&dotatypes::Item> = d.filtered_items.iter().map(|&i| &d.items[i]).collect();
    serde_json::to_string(&sub).unwrap_or_else(|e| {
        error!("Ошибка сериализации предметов: {}", e);
        String::new()
    })
}

fn preset_json_str() -> String {
    let names = presets::Preset::load_names();
    serde_json::to_string(&names).unwrap_or_else(|e| {
        error!("Ошибка сериализации списка пресетов: {}", e);
        String::new()
    })
}

#[derive(Default, QObject)]
#[allow(non_snake_case)]
pub struct AppState {
    base: qt_base_class!(trait QObject),

    getHeroesJson: qt_method!(fn(&self) -> QString),
    getItemsJson: qt_method!(fn(&self) -> QString),
    getPresetNamesJson: qt_method!(fn(&self) -> QString),
    getStatusMessage: qt_method!(fn(&self) -> QString),
    filterHeroes: qt_method!(fn(&self, query: String)),
    filterItems: qt_method!(fn(&self, query: String)),
    editHero: qt_method!(fn(&self, index: i32, username: String, skillsJson: String)),
    editItem: qt_method!(fn(&self, index: i32, username: String)),
    savePreset: qt_method!(fn(&self, name: String)),
    loadPreset: qt_method!(fn(&self, filename: String)),
    saveChanges: qt_method!(fn(&self)),
    resetAll: qt_method!(fn(&self)),
    openPresetsFolder: qt_method!(fn(&self)),
    setDotaPath: qt_method!(fn(&self, path: String)),
    getDotaPath: qt_method!(fn(&self) -> QString),
}

#[allow(non_snake_case)]
impl AppState {
    fn getHeroesJson(&self) -> QString {
        match app_data().lock() {
            Ok(d) => hero_json(&d).into(),
            Err(_) => {
                error!("Mutex poisoned при getHeroesJson");
                QString::default()
            }
        }
    }
    fn getItemsJson(&self) -> QString {
        match app_data().lock() {
            Ok(d) => item_json(&d).into(),
            Err(_) => {
                error!("Mutex poisoned при getItemsJson");
                QString::default()
            }
        }
    }
    fn getPresetNamesJson(&self) -> QString { preset_json_str().into() }
    fn getStatusMessage(&self) -> QString {
        match app_data().lock() {
            Ok(d) => d.status_message.clone().into(),
            Err(_) => {
                error!("Mutex poisoned при getStatusMessage");
                QString::default()
            }
        }
    }

    fn filterHeroes(&self, query: String) {
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при filterHeroes");
                return;
            }
        };
        debug!("Фильтр героев: \"{}\"", query);
        d.filter_hero_query = query.to_lowercase();
        d.filtered_heroes = if d.filter_hero_query.is_empty() {
            (0..d.heroes.len()).collect()
        } else {
            d.heroes
                .iter()
                .enumerate()
                .filter(|(_, h)| {
                    h.base.name.to_lowercase().contains(&d.filter_hero_query)
                        || h.base
                            .username
                            .as_ref()
                            .is_some_and(|u| u.to_lowercase().contains(&d.filter_hero_query))
                })
                .map(|(i, _)| i)
                .collect()
        };
        debug!("Найдено героев: {}", d.filtered_heroes.len());
    }

    fn filterItems(&self, query: String) {
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при filterItems");
                return;
            }
        };
        debug!("Фильтр предметов: \"{}\"", query);
        d.filter_item_query = query.to_lowercase();
        d.filtered_items = if d.filter_item_query.is_empty() {
            (0..d.items.len()).collect()
        } else {
            d.items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    item.entity.name.to_lowercase().contains(&d.filter_item_query)
                        || item
                            .entity
                            .username
                            .as_ref()
                            .is_some_and(|u| u.to_lowercase().contains(&d.filter_item_query))
                })
                .map(|(i, _)| i)
                .collect()
        };
        debug!("Найдено предметов: {}", d.filtered_items.len());
    }

    fn editHero(&self, index: i32, username: String, skillsJson: String) {
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при editHero");
                return;
            }
        };
        let ui = index as usize;
        let real = if ui < d.filtered_heroes.len() {
            d.filtered_heroes[ui]
        } else {
            warn!("editHero: индекс {} вне диапазона", index);
            return;
        };
        let hero = &mut d.heroes[real];
        hero.base.username = if username.is_empty() {
            None
        } else {
            Some(username)
        };
        if let Ok(skills) = serde_json::from_str::<Vec<serde_json::Value>>(&skillsJson) {
            for (i, sv) in skills.iter().enumerate() {
                if i < hero.skills.len() {
                    if let Some(c) = sv.get("custom").and_then(|v| v.as_str()) {
                        hero.skills[i].entity.username =
                            if c.is_empty() || c == "N/A" {
                                None
                            } else {
                                Some(c.into())
                            };
                    }
                }
            }
        }
        info!("Герой '{}' (index={}) сохранён в памяти", hero.base.name, real);
        d.status_message = "Герой сохранён".into();
    }

    fn editItem(&self, index: i32, username: String) {
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при editItem");
                return;
            }
        };
        let ui = index as usize;
        let real = if ui < d.filtered_items.len() {
            d.filtered_items[ui]
        } else {
            warn!("editItem: индекс {} вне диапазона", index);
            return;
        };
        d.items[real].entity.username = if username.is_empty() {
            None
        } else {
            Some(username)
        };
        info!(
            "Предмет '{}' (index={}) сохранён в памяти",
            d.items[real].entity.name, real
        );
        d.status_message = "Предмет сохранён".into();
    }

    fn savePreset(&self, name: String) {
        if name.is_empty() {
            return;
        }
        let d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при savePreset");
                return;
            }
        };
        let preset = presets::Preset::new(&name, d.heroes.clone(), d.items.clone());
        drop(d);
        preset.save();
        info!("Пресет '{}' сохранён", name);
        set_status(&format!("Пресет '{}' сохранён", name));
    }

    fn loadPreset(&self, filename: String) {
        info!("Загрузка пресета '{}'", filename);
        let preset = presets::Preset::load(&filename);
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при loadPreset");
                return;
            }
        };
        d.heroes = preset.heroes;
        d.items = preset.items;
        d.filter_hero_query.clear();
        d.filter_item_query.clear();
        d.filtered_heroes = (0..d.heroes.len()).collect();
        d.filtered_items = (0..d.items.len()).collect();
        info!(
            "Пресет '{}' загружен: {} героев, {} предметов",
            filename,
            d.heroes.len(),
            d.items.len()
        );
        d.status_message = format!("Пресет '{}' загружен", filename);
    }

    fn saveChanges(&self) {
        info!("Начало сохранения изменений");
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при saveChanges");
                return;
            }
        };
        let content = match std::fs::read_to_string("data/abilities_russian.txt") {
            Ok(c) => c,
            Err(e) => {
                error!("Не удалось прочитать abilities_russian.txt: {}", e);
                d.status_message = "Не удалось прочитать abilities_russian.txt".into();
                return;
            }
        };
        debug!("abilities_russian.txt прочитан: {} байт", content.len());
        let mut kv = kvparser::parse(&content);
        debug!("KV записей после парсинга: {}", kv.len());
        for hero in &d.heroes {
            kv.extend(hero.to_key_pairs());
        }
        for item in &d.items {
            let (k, v) = item.to_key_pair();
            kv.insert(k, v);
        }
        debug!("KV записей после наложения изменений: {}", kv.len());
        let hj = serde_json::to_string_pretty(
            &d.heroes.iter().map(|h| h.to_dict()).collect::<Vec<_>>(),
        )
        .unwrap_or_else(|e| {
            error!("Ошибка сериализации hero_tags.json: {}", e);
            String::new()
        });
        let ij = serde_json::to_string_pretty(
            &d.items.iter().map(|i| i.to_dict()).collect::<Vec<_>>(),
        )
        .unwrap_or_else(|e| {
            error!("Ошибка сериализации items_tags.json: {}", e);
            String::new()
        });

        if let Err(e) = std::fs::write("data/hero_tags.json", &hj) {
            error!("Не удалось записать hero_tags.json: {}", e);
        } else {
            debug!("hero_tags.json записан");
        }
        if let Err(e) = std::fs::write("data/items_tags.json", &ij) {
            error!("Не удалось записать items_tags.json: {}", e);
        } else {
            debug!("items_tags.json записан");
        }

        let out = kvparser::unparse(&kv, "russian");
        if let Err(e) = std::fs::write("data/abilities_russian.txt", &out) {
            error!("Не удалось записать abilities_russian.txt: {}", e);
        } else {
            debug!("abilities_russian.txt записан");
        }

        let vpk = match d.config.vpk_path() {
            Some(p) => p,
            None => {
                warn!("Путь к Dota 2 не указан, VPK не обновлён");
                d.status_message = "Путь к Dota 2 не указан".into();
                return;
            }
        };
        drop(d);
        match vpk::replace_file(
            &vpk,
            "resource/localization/abilities_russian.txt",
            "data/abilities_russian.txt",
        ) {
            Ok(_) => {
                info!("Изменения сохранены в VPK: {}", vpk);
                set_status("Изменения сохранены в VPK");
            }
            Err(e) => {
                error!("Ошибка сохранения VPK: {}", e);
                set_status(&format!("Ошибка сохранения VPK: {}", e));
            }
        }
    }

    fn resetAll(&self) {
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при resetAll");
                return;
            }
        };
        for hero in &mut d.heroes {
            hero.base.username = None;
            for skill in &mut hero.skills {
                skill.entity.username = None;
            }
        }
        for item in &mut d.items {
            item.entity.username = None;
        }
        info!("Все настройки сброшены");
        d.status_message = "Все настройки сброшены".into();
    }

    fn openPresetsFolder(&self) {
        let p = std::env::current_dir()
            .unwrap_or_default()
            .join("presets");
        let path_str = p.to_str().unwrap_or(".");
        match std::process::Command::new("xdg-open").arg(path_str).spawn() {
            Ok(_) => debug!("Открыта папка пресетов: {}", path_str),
            Err(e) => warn!("Не удалось открыть папку пресетов: {}", e),
        }
    }

    fn setDotaPath(&self, path: String) {
        let mut d = match app_data().lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Mutex poisoned при setDotaPath");
                return;
            }
        };
        d.config.dota_directory = Some(path.clone());
        d.config.save();
        info!("Путь Dota 2 обновлён: {}", path);
        d.status_message = format!("Путь Dota 2 обновлён: {}", path);
    }

    fn getDotaPath(&self) -> QString {
        match app_data().lock() {
            Ok(d) => d.config.dota_directory.clone().unwrap_or_default().into(),
            Err(_) => {
                error!("Mutex poisoned при getDotaPath");
                QString::default()
            }
        }
    }
}

fn setup_logger(level: &str) {
    let log_level: log::LevelFilter = level
        .parse()
        .unwrap_or(log::LevelFilter::Info);

    let log_path = config::log_path();
    if let Some(parent) = log_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {:5}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

    match fern::log_file(&log_path) {
        Ok(log_file) => {
            dispatch = dispatch.chain(log_file);
        }
        Err(e) => {
            eprintln!("Не удалось открыть файл лога {}: {}", log_path.display(), e);
        }
    }

    dispatch.apply().unwrap_or_else(|e| {
        eprintln!("Ошибка инициализации логгера: {}", e);
    });

    info!("Логгер инициализирован: level={}, file={}", level, log_path.display());
}

fn main() {
    let cfg = config::Config::load();
    setup_logger(&cfg.logger_lvl);

    info!("Запуск приложения");
    let mut engine = QmlEngine::new();
    let app = RefCell::new(AppState::default());
    let pinned = unsafe { QObjectPinned::new(&app) };
    engine.set_object_property("appState".into(), pinned);
    engine.load_file("qml/main.qml".into());
    info!("QML загружен, запуск event loop");
    engine.exec();
    info!("Приложение завершено");
}
