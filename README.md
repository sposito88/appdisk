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

## Instalação

### Via Cargo (Recomendado)

Se você tem o Rust instalado:

```bash
cargo install appdisk
```

### Via Script de Instalação

# Clone o repositório
git clone https://github.com/sposito88/appdisk.git
cd appdisk

# Execute o script de instalação
chmod +x install.sh
./install.sh

### Via Docker

```bash
# Construir a imagem
docker build -t appdisk .

# Executar
docker run -it --rm -v /path/to/analyze:/data appdisk /data
```

### Via Pacote Debian (.deb)

Para sistemas baseados em Debian/Ubuntu:

```bash
# Baixe o pacote mais recente das releases
wget https://github.com/sposito88/appdisk/releases/latest/download/appdisk_0.1.0_amd64.deb

# Instale o pacote
sudo dpkg -i appdisk_0.1.0_amd64.deb
```

## Uso

### Comando Básico

```bash
appdisk [DIRETÓRIO]
```

Se nenhum diretório for especificado, o diretório atual será analisado.

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
| q | Sai do programa |

## Desenvolvimento

### Pré-requisitos

- Rust 1.75 ou superior
- Cargo

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

### Testes

```bash
cargo test
```

## Contribuindo

Contribuições são bem-vindas! Por favor, sinta-se à vontade para enviar pull requests.

1. Fork o projeto
2. Crie sua branch de feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## Autor

Alan Sposito - [spositoalan@gmail.com](mailto:spositoalan@gmail.com)

Projeto: [https://github.com/sposito88/appdisk](https://github.com/sposito88/appdisk)

## Agradecimentos

- Inspirado no [NCDU](https://dev.yorhel.nl/ncdu)
- Construído com [Ratatui](https://github.com/ratatui-org/ratatui)
