use std::fs;
use std::path::Path;

pub fn quarantine_files(dupes: &std::collections::HashMap<String, Vec<String>>) {
    fs::create_dir_all("quarantine").ok();

    for (_hash, files) in dupes {
        for file in &files[1..] { // keep one, move others
            if let Some(name) = Path::new(file).file_name() {
                let dest = format!("quarantine/{}", name.to_string_lossy());
                fs::rename(file, dest).ok();
            }
        }
    }
}
