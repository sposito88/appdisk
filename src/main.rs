use anyhow::Result;
use clap::Parser;

mod app;
mod scanner;
mod ui;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Diretório para analisar
    #[arg(default_value = ".")]
    diretorio: String,

    /// Nível máximo de profundidade da análise
    #[arg(short, long)]
    profundidade: Option<usize>,

    /// Arquivo de saída para exportação JSON
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Inicializa o scanner
    let scanner = scanner::Scanner::new(&args.diretorio, args.profundidade);
    
    // Se foi especificado um arquivo de saída, exporta em JSON
    if let Some(output) = args.output {
        let resultado = scanner.scan()?;
        std::fs::write(output, serde_json::to_string_pretty(&resultado)?)?;
        return Ok(());
    }

    // Caso contrário, inicia a interface TUI
    let app = app::App::new(scanner);
    ui::run(app)?;

    Ok(())
} 