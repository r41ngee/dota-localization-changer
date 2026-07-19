use log::{debug, error, info};

const EMPTY_ABILITIES_CONTENT: &str = "\"lang\"\n{\n\t\"Language\" \"russian\"\n\t\"Tokens\"\n\t{\n\t}\n}\n";

pub fn create_vpk_with_empty_abilities(vpk_path: &str) -> Result<(), String> {
    let temp_dir = std::env::temp_dir().join(format!("dota_lc_vpk_new_{}", std::process::id()));
    if let Err(e) = std::fs::remove_dir_all(&temp_dir) {
        debug!("Очистка временной директории (можно игнорировать): {}", e);
    }

    let file_path = temp_dir.join("resource/localization/abilities_russian.txt");
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Ошибка создания папки: {}", e))?;
    }
    std::fs::write(&file_path, EMPTY_ABILITIES_CONTENT)
        .map_err(|e| format!("Ошибка записи {}: {}", file_path.display(), e))?;
    debug!("Создан {}", file_path.display());

    let new_vpk = valve_pak::VPK::from_directory(&temp_dir)
        .map_err(|e| format!("Ошибка создания VPK: {}", e))?;
    new_vpk
        .save(vpk_path)
        .map_err(|e| format!("Ошибка сохранения VPK: {}", e))?;
    info!("Создан новый VPK: {}", vpk_path);

    if let Err(e) = std::fs::remove_dir_all(&temp_dir) {
        error!("Не удалось удалить временную директорию {}: {}", temp_dir.display(), e);
    }
    Ok(())
}

pub fn extract_file(vpk_path: &str, vpk_internal_path: &str, dest_path: &str) -> Result<(), String> {
    info!("Открытие VPK: {}", vpk_path);
    let vpk = valve_pak::VPK::open(vpk_path)
        .map_err(|e| format!("Ошибка открытия VPK: {}", e))?;

    let mut file = vpk
        .get_file(vpk_internal_path)
        .map_err(|e| format!("Ошибка чтения {} в VPK: {}", vpk_internal_path, e))?;

    if let Some(parent) = std::path::Path::new(dest_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Ошибка создания папки: {}", e))?;
    }

    file.save(dest_path)
        .map_err(|e| format!("Ошибка сохранения {}: {}", dest_path, e))?;
    info!("Извлечён {} -> {}", vpk_internal_path, dest_path);
    Ok(())
}

pub fn replace_file(vpk_path: &str, target_path: &str, source_file: &str) -> Result<(), String> {
    info!("Открытие VPK: {}", vpk_path);
    let vpk = valve_pak::VPK::open(vpk_path)
        .map_err(|e| format!("Ошибка открытия VPK: {}", e))?;

    let temp_dir = std::env::temp_dir().join(format!("dota_lc_vpk_{}", std::process::id()));
    if let Err(e) = std::fs::remove_dir_all(&temp_dir) {
        debug!("Очистка временной директории (можно игнорировать): {}", e);
    }

    let paths: Vec<String> = vpk.file_paths().cloned().collect();
    debug!("Файлов в VPK: {}", paths.len());

    for path in &paths {
        let target = temp_dir.join(path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Ошибка создания папки: {}", e))?;
        }
        let mut file = vpk
            .get_file(path)
            .map_err(|e| format!("Ошибка чтения {}: {}", path, e))?;
        file.save(&target)
            .map_err(|e| format!("Ошибка сохранения {}: {}", path, e))?;
    }
    debug!("Все файлы извлечены во временную директорию");

    let dest = temp_dir.join(target_path);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Ошибка создания папки: {}", e))?;
    }
    std::fs::copy(source_file, &dest)
        .map_err(|e| format!("Ошибка копирования {}: {}", source_file, e))?;
    debug!("{} скопирован в {}", source_file, dest.display());

    let new_vpk = valve_pak::VPK::from_directory(&temp_dir)
        .map_err(|e| format!("Ошибка создания VPK: {}", e))?;
    new_vpk
        .save(vpk_path)
        .map_err(|e| format!("Ошибка сохранения VPK: {}", e))?;
    info!("VPK сохранён: {}", vpk_path);

    if let Err(e) = std::fs::remove_dir_all(&temp_dir) {
        error!("Не удалось удалить временную директорию {}: {}", temp_dir.display(), e);
    } else {
        debug!("Временная директория удалена");
    }

    Ok(())
}
