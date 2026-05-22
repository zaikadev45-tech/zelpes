use std::path::Path;
use walkdir::WalkDir;

pub fn back_names(pasta: &Path, extensao: &str) -> Vec<String> {
    WalkDir::new(pasta)
        .into_iter()
        .flatten()
        .filter(|entry| {
            // ignora a pasta .zelpes para evitar loop
            !entry.path().starts_with(pasta.join(".zelpes"))
        })
        .filter(|entry| {
            entry.path().extension()
                .map(|ext| ext.to_string_lossy().to_lowercase() == extensao.to_lowercase())
                .unwrap_or(false)
        })
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect()
}

pub fn arquivo_ext(pasta: &Path, extensao: &str) -> bool {
    WalkDir::new(pasta)
        .into_iter()
        .flatten()
        .filter(|entry| !entry.path().starts_with(pasta.join(".zelpes")))
        .any(|entry| {
            entry.path().extension()
                .map(|ext| ext.to_string_lossy().to_lowercase() == extensao.to_lowercase())
                .unwrap_or(false)
        })
}
