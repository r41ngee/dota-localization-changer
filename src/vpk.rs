pub fn replace_file(vpk_path: &str, target_path: &str, source_file: &str) -> Result<(), String> {
    let vpk = valve_pak::VPK::open(vpk_path)
        .map_err(|e| format!("Ошибка открытия VPK: {}", e))?;

    let temp_dir = std::env::temp_dir().join(format!("dota_lc_vpk_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp_dir);

    let paths: Vec<String> = vpk.file_paths().cloned().collect();

    for path in &paths {
        let target = temp_dir.join(path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Ошибка создания папки: {}", e))?;
        }
        let mut file = vpk.get_file(path)
            .map_err(|e| format!("Ошибка чтения {}: {}", path, e))?;
        file.save(&target)
            .map_err(|e| format!("Ошибка сохранения {}: {}", path, e))?;
    }

    let dest = temp_dir.join(target_path);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Ошибка создания папки: {}", e))?;
    }
    std::fs::copy(source_file, &dest)
        .map_err(|e| format!("Ошибка копирования {}: {}", source_file, e))?;

    let new_vpk = valve_pak::VPK::from_directory(&temp_dir)
        .map_err(|e| format!("Ошибка создания VPK: {}", e))?;
    new_vpk.save(vpk_path)
        .map_err(|e| format!("Ошибка сохранения VPK: {}", e))?;

    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(())
}
