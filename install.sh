#!/bin/bash

# Verifica se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "Instalando Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
fi

# Compila e instala o aplicativo
echo "Compilando appdisk..."
cargo build --release

# Copia o binário para /usr/local/bin
echo "Instalando appdisk..."
sudo cp target/release/appdisk /usr/local/bin/
sudo chmod +x /usr/local/bin/appdisk

echo "Instalação concluída! Use 'appdisk' para executar o programa." 