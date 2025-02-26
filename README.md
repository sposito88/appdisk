# Appdisk

Um analisador de uso de disco interativo com interface TUI (Terminal User Interface), inspirado no NCDU. Desenvolvido em Rust para garantir alta performance e eficiência na análise de diretórios.

## Características

- 🚀 Análise rápida e eficiente de diretórios
- 📊 Interface interativa no terminal
- 🔄 Múltiplas opções de ordenação (tamanho, nome, data)
- 📁 Navegação hierárquica entre diretórios
- 📈 Barra de rolagem visual
- 🔍 Suporte a arquivos ocultos
- 💾 Exportação para JSON
- 🗑️ Deleção segura de arquivos e diretórios

## Instalação

### Via Pacote Debian (.deb) - Recomendado para Ubuntu/Debian
```bash
# Baixe o pacote mais recente
wget https://github.com/sposito88/appdisk/releases/latest/download/appdisk_0.1.0_amd64.deb

# Instale o pacote
sudo dpkg -i appdisk_0.1.0_amd64.deb

# Se houver dependências faltando, execute:
sudo apt-get install -f
```

### Via Cargo
```bash
# Instale o Rust se ainda não tiver
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instale o appdisk
cargo install appdisk
```

### Via Script de Instalação
```bash
# Clone o repositório
git clone https://github.com/sposito88/appdisk.git
cd appdisk

# Execute o script de instalação
chmod +x install.sh
./install.sh
```

### Via Docker
```bash
# Construir a imagem
docker build -t appdisk .

# Executar analisando um diretório específico
docker run -it --rm -v /caminho/para/analisar:/data appdisk /data
```

## Uso

### Comando Básico
```bash
# Analisa o diretório atual
appdisk

# Analisa um diretório específico
appdisk /caminho/do/diretorio
```

### Opções
```bash
appdisk -h                    # Mostra ajuda
appdisk -d <nível>           # Define profundidade máxima
appdisk -o <arquivo.json>     # Exporta resultados para JSON
```

### Teclas de Atalho

| Tecla | Função |
|-------|--------|
| ↑ | Move seleção para cima |
| ↓ | Move seleção para baixo |
| Enter | Entra no diretório selecionado |
| Backspace | Volta ao diretório anterior |
| s | Alterna modo de ordenação (tamanho/nome/data) |
| h | Mostra/oculta arquivos ocultos |
| d | Deleta arquivo/diretório selecionado (com confirmação) |
| q | Sai do programa |

## Desenvolvimento

### Pré-requisitos
- Rust 1.75 ou superior
- Cargo
- (Opcional) Docker para construção de container

### Compilação
```bash
# Clone o repositório
git clone https://github.com/sposito88/appdisk.git
cd appdisk

# Compile em modo debug
cargo build

# Ou em modo release
cargo build --release
```

### Gerando pacote .deb
```bash
# Instale cargo-deb
cargo install cargo-deb

# Gere o pacote
cargo deb

# O pacote será gerado em target/debian/
```

### Testes
```bash
cargo test
```

## Contribuindo

Contribuições são bem-vindas! Por favor, sinta-se à vontade para enviar pull requests.

1. Fork o projeto
2. Crie sua branch de feature (`git checkout -b feature/NovaFuncionalidade`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/NovaFuncionalidade`)
5. Abra um Pull Request

## Segurança

- A função de deleção sempre pede confirmação
- Arquivos e diretórios são verificados antes da deleção
- Permissões são respeitadas
- Histórico de navegação é mantido

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## Autor

Alan Sposito - [spositoalan@gmail.com](mailto:spositoalan@gmail.com)

Projeto: [https://github.com/sposito88/appdisk](https://github.com/sposito88/appdisk)

## Agradecimentos

- Inspirado no [NCDU](https://dev.yorhel.nl/ncdu)
- Construído com [Ratatui](https://github.com/ratatui-org/ratatui)
- Comunidade Rust