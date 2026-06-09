![zelpes](https://raw.githubusercontent.com/zaikadev45-tech/zelpes/test/benner.png)

# zelpes
zelpes é um bot que cuida do diretório que foi executado

##
### Como usar:
*mexer no sdcard do celular?*
cd /sdcard
~/zelpes

*mexer só na pasta projetos?*
cd exemplo_projeto
~/zelpes

`` só é preciso executar o binário dentro da pasta aonde quer deixar o bot mexendo``



##
### O que ele faz
- Monitora o diretório atual a cada 5s
- Move arquivos por extensão:
  - `.kdbx` → cofre/
  - `.zip`, `.tar` → backup/
  - `.txt`, `.apk`, `.log` → tmp/
- Extensões configuráveis em `.zelpes/config.zik`

##
### Instalação
cargo build --release

##
### Dependências
- walkdir
 
