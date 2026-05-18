use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn registrar(logs_dir: &Path, texto: &str, file: &str) {
    let logger = logs_dir.join("movement.log");

    let mut arquivo = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&logger)
        .expect("[ERRO] ao abrir / criar arquivo");
    writeln!(arquivo, "[LOG]=FILE: {file} / {texto}").expect("[ERRO] erro ao registrar no arquivo");
}
