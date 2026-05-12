use std::path::{PathBuf, Path};
use std::fs;

use crate::ConfigZelpes;
use crate::brain::logger;

pub enum FileManager{
    LocalCofre,
    LocalBackup,
    LocalLogs,
    LocalTmp,
}

impl FileManager {
    fn locais(etx: &str) -> Self {
        match etx {
            "kdbx" => FileManager::LocalCofre,
            "zip" | "tar" => FileManager::LocalBackup,
            "log" => FileManager::LocalLogs,
            "txt" => FileManager::LocalTmp,
            _ => panic!("ERRO na etx do arquivo"),
        }
    }

    fn mover(&self, config: &ConfigZelpes, arquivo: &PathBuf) {
        let file = arquivo.file_name().unwrap();
        let nome: &str = file.to_str().unwrap();

        let destino = match self {
            FileManager::LocalCofre => {
                logger::registrar("movido para pasta Cofre", nome);
                config.cofre.join(file)
            }
            FileManager::LocalBackup => {
                logger::registrar("movido para pasta backup", nome);
                config.backup.join(file)
            }
            FileManager::LocalLogs => {
                logger::registrar("log movido para o TMP", nome);
                config.tmp.join(file)
            }
            FileManager::LocalTmp => {
                logger::registrar("movido para tmp", nome);
                config.tmp.join(file)
            }
        };
        fs::create_dir_all(destino.parent().unwrap()).ok();
        fs::rename(arquivo, &destino).ok();
    }
}

pub fn brain_file(files: Vec<String>, config: &ConfigZelpes) {
    for file_name in files {
        let arquivo = PathBuf::from(&file_name);
        
        let ext = arquivo
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let local = FileManager::locais(ext);
        
        local.mover(config, &arquivo);
        
        println!("[+]={} movido", file_name);
    }
}
