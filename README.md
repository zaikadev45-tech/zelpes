![zelpes](https://raw.githubusercontent.com/zaikadev45-tech/zelpes/test/benner.png)

##
### zelpes
zelpes é um bot que cuida do diretório que foi executado

##
### Como usar:
*mexer no sdcard do celular?*
- cd /sdcard
- ~/zelpes

*mexer só na pasta projetos?*
- cd exemplo_projeto
- ~/zelpes

`` só é preciso executar o binário dentro da pasta aonde quer deixar o bot mexendo, e ele precisa de permissão pra mover/deletar no diretório``



##
### O que ele faz:
ele já tem configurações padrões como:
- `.txt / .log ` vai pra pasta .zelpes/tmp e depois é deletado,
- `kdbx` vai pro .zelpes/cofre
- Extensões que eu não configurei vai pro /tmp então **CUIDADO** ainda vou ajustar o bot pra decidir qual ação tomar conforme o arquivo config.zik

- Extensões configuráveis em `.zelpes/config.zik`
coloque só o nome da extensão 

##
### Instalação
cargo build --release

##
### Dependências
- walkdir
 
