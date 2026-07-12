use log::{debug, error, info};

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
