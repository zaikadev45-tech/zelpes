![descrição](banner.png)

# zelpes
zelpes é um bot que cuida do diretório que foi executado

## Como usar
cd /pasta/que/quer/monitorar
zelpes

## O que ele faz
- Monitora o diretório atual a cada 5s
- Move arquivos por extensão:
  - `.kdbx` → cofre/
  - `.zip`, `.tar` → backup/
  - `.txt`, `.apk`, `.log` → tmp/
- Extensões configuráveis em `.zelpes/config.zik`

## Instalação
cargo build --release

## Dependências
- walkdir
 
