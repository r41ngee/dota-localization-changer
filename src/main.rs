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
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse {}: {}", path, e))
}

fn app_data() -> &'static Mutex<AppData> {
    static DATA: OnceLock<Mutex<AppData>> = OnceLock::new();
    DATA.get_or_init(|| {
        let cfg = config::Config::load();
        let heroes = load_json::<Vec<dotatypes::Hero>>("data/hero_tags.json").unwrap_or_default();
        let items = load_json::<Vec<dotatypes::Item>>("data/items_tags.json").unwrap_or_default();
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
    app_data().lock().unwrap().status_message = msg.to_string();
}

fn hero_json(d: &AppData) -> String {
    let sub: Vec<&dotatypes::Hero> = d.filtered_heroes.iter().map(|&i| &d.heroes[i]).collect();
    serde_json::to_string(&sub).unwrap_or_default()
}

fn item_json(d: &AppData) -> String {
    let sub: Vec<&dotatypes::Item> = d.filtered_items.iter().map(|&i| &d.items[i]).collect();
    serde_json::to_string(&sub).unwrap_or_default()
}

fn preset_json_str() -> String {
    let names = presets::Preset::load_names();
    serde_json::to_string(&names).unwrap_or_default()
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
    fn getHeroesJson(&self) -> QString { let d = app_data().lock().unwrap(); hero_json(&d).into() }
    fn getItemsJson(&self) -> QString { let d = app_data().lock().unwrap(); item_json(&d).into() }
    fn getPresetNamesJson(&self) -> QString { preset_json_str().into() }
    fn getStatusMessage(&self) -> QString { app_data().lock().unwrap().status_message.clone().into() }

    fn filterHeroes(&self, query: String) {
        let mut d = app_data().lock().unwrap();
        d.filter_hero_query = query.to_lowercase();
        d.filtered_heroes = if d.filter_hero_query.is_empty() {
            (0..d.heroes.len()).collect()
        } else {
            d.heroes.iter().enumerate()
                .filter(|(_, h)| h.base.name.to_lowercase().contains(&d.filter_hero_query)
                    || h.base.username.as_ref().is_some_and(|u| u.to_lowercase().contains(&d.filter_hero_query)))
                .map(|(i, _)| i).collect()
        };
    }

    fn filterItems(&self, query: String) {
        let mut d = app_data().lock().unwrap();
        d.filter_item_query = query.to_lowercase();
        d.filtered_items = if d.filter_item_query.is_empty() {
            (0..d.items.len()).collect()
        } else {
            d.items.iter().enumerate()
                .filter(|(_, item)| item.entity.name.to_lowercase().contains(&d.filter_item_query)
                    || item.entity.username.as_ref().is_some_and(|u| u.to_lowercase().contains(&d.filter_item_query)))
                .map(|(i, _)| i).collect()
        };
    }

    fn editHero(&self, index: i32, username: String, skillsJson: String) {
        let mut d = app_data().lock().unwrap();
        let ui = index as usize;
        let real = if ui < d.filtered_heroes.len() { d.filtered_heroes[ui] } else { return };
        let hero = &mut d.heroes[real];
        hero.base.username = if username.is_empty() { None } else { Some(username) };
        if let Ok(skills) = serde_json::from_str::<Vec<serde_json::Value>>(&skillsJson) {
            for (i, sv) in skills.iter().enumerate() {
                if i < hero.skills.len() {
                    if let Some(c) = sv.get("custom").and_then(|v| v.as_str()) {
                        hero.skills[i].entity.username = if c.is_empty() || c == "N/A" { None } else { Some(c.into()) };
                    }
                }
            }
        }
        d.status_message = "Герой сохранён".into();
    }

    fn editItem(&self, index: i32, username: String) {
        let mut d = app_data().lock().unwrap();
        let ui = index as usize;
        let real = if ui < d.filtered_items.len() { d.filtered_items[ui] } else { return };
        d.items[real].entity.username = if username.is_empty() { None } else { Some(username) };
        d.status_message = "Предмет сохранён".into();
    }

    fn savePreset(&self, name: String) {
        if name.is_empty() { return; }
        let d = app_data().lock().unwrap();
        let preset = presets::Preset::new(&name, d.heroes.clone(), d.items.clone());
        preset.save();
        drop(d);
        set_status(&format!("Пресет '{}' сохранён", name));
    }

    fn loadPreset(&self, filename: String) {
        let preset = presets::Preset::load(&filename);
        let mut d = app_data().lock().unwrap();
        d.heroes = preset.heroes;
        d.items = preset.items;
        d.filter_hero_query.clear();
        d.filter_item_query.clear();
        d.filtered_heroes = (0..d.heroes.len()).collect();
        d.filtered_items = (0..d.items.len()).collect();
        d.status_message = format!("Пресет '{}' загружен", filename);
    }

    fn saveChanges(&self) {
        let mut d = app_data().lock().unwrap();
        let content = match std::fs::read_to_string("data/abilities_russian.txt") {
            Ok(c) => c,
            Err(_) => { d.status_message = "Не удалось прочитать abilities_russian.txt".into(); return; }
        };
        let mut kv = kvparser::parse(&content);
        for hero in &d.heroes { kv.extend(hero.to_key_pairs()); }
        for item in &d.items { let (k, v) = item.to_key_pair(); kv.insert(k, v); }
        let hj = serde_json::to_string_pretty(&d.heroes.iter().map(|h| h.to_dict()).collect::<Vec<_>>()).unwrap_or_default();
        let ij = serde_json::to_string_pretty(&d.items.iter().map(|i| i.to_dict()).collect::<Vec<_>>()).unwrap_or_default();
        let _ = std::fs::write("data/hero_tags.json", &hj);
        let _ = std::fs::write("data/items_tags.json", &ij);
        let out = kvparser::unparse(&kv, "russian");
        let _ = std::fs::write("data/abilities_russian.txt", &out);
        let vpk = match d.config.vpk_path() {
            Some(p) => p,
            None => { d.status_message = "Путь к Dota 2 не указан".into(); return; }
        };
        drop(d);
        match vpk::replace_file(&vpk, "resource/localization/abilities_russian.txt", "data/abilities_russian.txt") {
            Ok(_) => set_status("Изменения сохранены в VPK"),
            Err(e) => set_status(&format!("Ошибка сохранения VPK: {}", e)),
        }
    }

    fn resetAll(&self) {
        let mut d = app_data().lock().unwrap();
        for hero in &mut d.heroes {
            hero.base.username = None;
            for skill in &mut hero.skills { skill.entity.username = None; }
        }
        for item in &mut d.items { item.entity.username = None; }
        d.status_message = "Все настройки сброшены".into();
    }

    fn openPresetsFolder(&self) {
        let p = std::env::current_dir().unwrap_or_default().join("presets");
        let _ = std::process::Command::new("xdg-open").arg(p.to_str().unwrap_or(".")).spawn();
    }

    fn setDotaPath(&self, path: String) {
        let mut d = app_data().lock().unwrap();
        d.config.dota_directory = Some(path.clone());
        d.config.save();
        d.status_message = format!("Путь Dota 2 обновлён: {}", path);
    }

    fn getDotaPath(&self) -> QString {
        let d = app_data().lock().unwrap();
        d.config.dota_directory.clone().unwrap_or_default().into()
    }
}

fn main() {
    let mut engine = QmlEngine::new();
    let app = RefCell::new(AppState::default());
    let pinned = unsafe { QObjectPinned::new(&app) };
    engine.set_object_property("appState".into(), pinned);
    engine.load_file("qml/main.qml".into());
    engine.exec();
}
