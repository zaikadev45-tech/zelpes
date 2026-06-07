use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn config(zelpes: &Path) -> Vec<String> {
    let config = zelpes.join("config.zik");

    if config.exists() {
        let exts = zik(&config);
        return exts;
    } else {
        let arquivo = File::create(&config).expect("ERRI");

        {
            let mut init = BufWriter::new(arquivo);

            writeln!(init, "txt").expect("ERRO ESCRITA");
            writeln!(init, "zip").expect("ERRO ESCRITA");
        }

        let exts = zik(&config);
        return exts;
    }
    
}

fn zik(path: &Path) -> Vec<String> {
    let arquivo = File::open(path).expect("ERRO");
    let mut exts: Vec<String> = Vec::new();
    let leitor = BufReader::new(&arquivo);
    
    for linha in leitor.lines() {
        let linha = linha.unwrap();
        exts.push(linha);
    } 

    exts
}
