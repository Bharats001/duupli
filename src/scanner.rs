use walkdir::WalkDir;

pub fn scan_dir(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().map(|m| m.is_file()).unwrap_or(false))
        .map(|e| e.path().to_string_lossy().to_string())
        .collect()
}
