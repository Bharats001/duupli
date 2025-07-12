use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs, path::Path};

pub fn hash_file_sha256(path: &str) -> Option<String> {
    let data = fs::read(path).ok()?;
    let hash = Sha256::digest(&data);
    Some(format!("{:x}", hash))
}

pub fn find_duplicates(files: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    files.par_iter().filter_map(|f| {
        hash_file_sha256(f).map(|hash| (hash, f.clone()))
    }).collect::<Vec<_>>()
    .into_iter()
    .for_each(|(hash, file)| {
        map.entry(hash).or_default().push(file);
    });

    map.retain(|_, v| v.len() > 1); // keep only duplicates
    map
}
