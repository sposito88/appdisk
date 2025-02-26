use crate::app::{App, PopupState};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, BorderType, Clear},
    Frame, Terminal,
    prelude::Alignment,
};
use std::io::stdout;

pub fn run(mut app: App) -> Result<()> {
    // Configuração do terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Realiza o escaneamento inicial
    app.scan()?;

    // Loop principal
    loop {
        let terminal_height = terminal.size()?.height as usize - 6; // -6 para cabeçalho e rodapé
        app.update_scroll(terminal_height);
        
        terminal.draw(|frame| ui(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    if app.popup_state != PopupState::Hidden {
                        app.hide_popup();
                    } else {
                        break;
                    }
                },
                KeyCode::Char('s') => app.toggle_sort_order(),
                KeyCode::Char('h') => app.toggle_hidden(),
                KeyCode::Char('d') => {
                    if app.popup_state == PopupState::Hidden {
                        app.show_delete_confirmation();
                    }
                },
                KeyCode::Up => {
                    if app.selected_index > 0 {
                        app.selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if let Some(entries) = &app.entries {
                        if app.selected_index < entries.children.len().saturating_sub(1) {
                            app.selected_index += 1;
                        }
                    }
                }
                KeyCode::Enter => {
                    if app.popup_state == PopupState::DeleteConfirmation {
                        app.confirm_delete()?;
                    } else {
                        app.enter_directory()?;
                    }
                },
                KeyCode::Backspace => {
                    app.go_back()?;
                },
                KeyCode::Esc => {
                    if app.popup_state != PopupState::Hidden {
                        app.hide_popup();
                    }
                },
                _ => {}
            }
        }
    }

    // Restaura o terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    // Cabeçalho
    let header_content = Line::from(vec![
        Span::raw("Diretório atual: "),
        Span::styled(
            app.current_path.to_string_lossy().to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]);
    
    let header = Paragraph::new(vec![header_content])
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // Lista de arquivos com barra de rolagem
    if let Some(entries) = &app.entries {
        let height = chunks[1].height as usize - 2; // -2 para as bordas
        let items: Vec<ListItem> = entries
            .children
            .iter()
            .skip(app.scroll_offset)
            .take(height)
            .enumerate()
            .map(|(i, entry)| {
                let size = format_size(entry.size);
                let prefix = if entry.is_dir { "📁 " } else { "📄 " };
                let name = entry.path.file_name().unwrap().to_string_lossy();
                let content = format!("{}{} ({})", prefix, name, size);
                
                let style = if i + app.scroll_offset == app.selected_index {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                
                ListItem::new(content).style(style)
            })
            .collect();

        let total_items = entries.children.len();
        let scroll_indicator = if total_items > height {
            let scroll_percentage = app.scroll_offset as f64 / (total_items - height) as f64;
            let scroll_pos = (height as f64 * scroll_percentage) as usize;
            let mut indicator: Vec<char> = "│".repeat(height).chars().collect();
            if scroll_pos < indicator.len() {
                indicator[scroll_pos] = '█';
            }
            indicator.into_iter().collect::<String>()
        } else {
            "│".repeat(height)
        };

        let files_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Arquivos")
                    .border_type(BorderType::Rounded),
            );

        let scroll_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(chunks[1]);

        frame.render_widget(files_list, inner_chunks[0]);
        frame.render_widget(
            Paragraph::new(scroll_indicator).block(scroll_block),
            inner_chunks[1],
        );
    }

    // Renderiza o popup de confirmação se necessário
    if app.popup_state == PopupState::DeleteConfirmation {
        let popup = render_delete_confirmation(frame.size(), app);
        frame.render_widget(Clear, popup); // Limpa a área do popup
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(Style::default().bg(Color::DarkGray));
        
        frame.render_widget(block, popup);

        let selected_name = app.entries.as_ref()
            .and_then(|entries| entries.children.get(app.selected_for_deletion?))
            .map(|entry| entry.path.file_name().unwrap_or_default().to_string_lossy().to_string())
            .unwrap_or_default();

        let is_dir = app.entries.as_ref()
            .and_then(|entries| entries.children.get(app.selected_for_deletion?))
            .map(|entry| entry.is_dir)
            .unwrap_or(false);

        let tipo = if is_dir { "diretório" } else { "arquivo" };
        let mensagem = format!("Tem certeza que deseja deletar o {} '{}'?\n\n[Enter] Confirmar | [Esc] Cancelar", 
            tipo, selected_name);

        let text = Paragraph::new(mensagem)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));

        frame.render_widget(text, popup);
    }

    // Rodapé
    let footer_text = vec![
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Sair | "),
        Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Ordenar | "),
        Span::styled("h", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Mostrar/Ocultar | "),
        Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Deletar | "),
        Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Navegar | "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Abrir | "),
        Span::styled("Backspace", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Voltar"),
    ];

    let footer = Paragraph::new(Line::from(footer_text))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded))
        .alignment(Alignment::Center);

    frame.render_widget(footer, chunks[2]);
}

fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

fn render_delete_confirmation(r: Rect, _app: &App) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(4),
            Constraint::Percentage(40),
        ])
        .split(r);

    let popup_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(40),
            Constraint::Percentage(30),
        ])
        .split(popup_layout[1])[1];

    popup_horizontal
} 