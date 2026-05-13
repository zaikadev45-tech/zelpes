use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs, thread};

pub struct ConfigZelpes {
    zelpes_dir: PathBuf,
    dir: PathBuf,

    cofre: PathBuf,
    backup: PathBuf,
    logs: PathBuf,
    tmp: PathBuf,
}

pub mod brain;

fn main() {
    println!("[#] zelpes inicializado...");

    let ext_s = ["txt", "kdbx"];
    let zelpes = ConfigZelpes::new();

    if !zelpes.zelpes_dir.exists() {
        ConfigZelpes::init_zelpes(&zelpes);
    }

    // sistema do zelpes
    let _bot_zelpes = thread::spawn(move || loop {
        if !zelpes.cofre.exists() {
            panic!("[!!] Pasta Cofre sumiu!");
        } else if !zelpes.backup.exists() {
            println!("[!!!] Pasta backup sumiu!");
            panic!("faça verificação manual na pasta .zelpes");
        } else if !zelpes.logs.exists() {
            panic!("[aviso][!] Pasta logs sumiu");
        } else if !zelpes.tmp.exists() {
            panic!("[!!] pasta tmp sumiu!");
        }

        //# gerenciar arquivo
        for ext in ext_s {
            let lista = brain::back_names(&zelpes.dir, ext);
            brain::brain_file(lista, &zelpes);
        }

        thread::sleep(Duration::from_secs(10));
        if brain::arquivo_ext(&zelpes.tmp, "txt") {
            let lista = brain::back_names(&zelpes.tmp, "txt");
            clear_tmp(&zelpes.tmp, lista);
        }
    });

    println!("[+] zalpes iniciado! aperte Ctrl+C para sair\n");
    loop {
        thread::sleep(Duration::from_secs(2));
    }
}

fn clear_tmp(dir_tmp: &PathBuf, list: Vec<String>) {
    for file_name in list {
        let arquivo = dir_tmp.join(&file_name);
        fs::remove_file(&arquivo)
            .expect("[ERRO] ao deletar arquivo");
        brain::logger::registrar("foi deletado", &file_name);
    }
}

impl ConfigZelpes {
    fn new() -> Self {
        let base = env::current_dir().unwrap();
        let zelpes = base.join(".zelpes");

        ConfigZelpes {
            cofre: zelpes.join("cofre"),
            backup: zelpes.join("backup"),
            logs: zelpes.join("logs"),
            tmp: zelpes.join("tmp"),

            dir: base,
            zelpes_dir: zelpes,
        }
    }

    fn init_zelpes(&self) {
        let pastas = [&self.cofre, &self.backup, &self.logs, &self.tmp];
        for &parte in &pastas {
            fs::create_dir_all(parte)
                .expect("[ERRO] na criação do {parte}");
        }
    }
}
