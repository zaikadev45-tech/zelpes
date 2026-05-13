use std::fs::OpenOptions;
use std::io::Write;

use crate::ConfigZelpes;

pub fn registrar(texto: &str, file: &str) {
    let struct_zelpes = ConfigZelpes::new();
    let logger = struct_zelpes.logs.join("movement.log");

    let mut arquivo = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&logger)
        .expect("[ERRO] ao abrir/criar arquivo");
    writeln!(arquivo, "[LOG]=FILE: {file} / {texto}").expect("[ERRO] erro ao registrar no arquivo");
}
