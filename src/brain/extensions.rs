use std::fs;
use std::path::Path;

//Pega nomes de pastas/arquivos no lugar
pub fn back_names(pasta: &Path, extensao: &str) -> Vec<String> {
    if let Ok(entries) = fs::read_dir(pasta) {
        entries
            .flatten()
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext.to_string_lossy().to_lowercase() == extensao.to_lowercase())
                    .unwrap_or(false)
            })
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect()
    } else {
        vec![]
    }
}

//verificar si tem arquivo com ext em uma pasta
pub fn arquivo_ext(pasta: &Path, extensao: &str) -> bool {
    if let Ok(entries) = fs::read_dir(pasta) {
        entries.flatten().any(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext.to_string_lossy().to_lowercase() == extensao.to_lowercase())
                .unwrap_or(false)
        })
    } else {
        false
    }
}
