use std::{fs, env, thread};
use std::time::Duration;
use std::path::PathBuf;

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
        }
        else if !zelpes.backup.exists() {
            println!("[!!!] Pasta backup sumiu!");
            panic!("faça verificação manual na pasta .zelpes");
         }
        else if !zelpes.logs.exists() {
            panic!("[aviso][!] Pasta logs sumiu");
        }
        else if !zelpes.tmp.exists() {
            panic!("[!!] pasta tmp sumiu!");
        }

        //# gerenciar arquivo
        for ext in ext_s {
            let lista = brain::back_names(&zelpes.dir, ext);
            brain::brain_file(lista, &zelpes);
        }

        thread::sleep(Duration::from_secs(5));
    });

    println!("[+] zalpes iniciado! aperte Ctrl+C para sair\n");
    loop {
        thread::sleep(Duration::from_secs(2));
    }
}

impl ConfigZelpes {
    fn new() -> Self {
        let base  = env::current_dir().unwrap();
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
        let zelpes_dir = &self.zelpes_dir;
        for &parte in &pastas {
            let criar = zelpes_dir.join(parte);
            fs::create_dir_all(criar)
                .expect("[ERRO] na criação do {parte}");
        }
    }
}

