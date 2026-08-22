use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use shared_types::OutputEvent;
use std::io::{self, BufRead};
use tokio::sync::mpsc;

pub async fn run_dashboard() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<OutputEvent>(100);

    std::thread::spawn(move || {
        let stdin = io::stdin();
        let handle = stdin.lock();

        for line in handle.lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(event) = serde_json::from_str::<OutputEvent>(&line) {
                if tx.blocking_send(event).is_err() {
                    break;
                }
            }
        }
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut logs: Vec<ListItem> =
        vec![
            ListItem::new("[READY] Listening for incoming NDJSON events via stdin...")
                .style(Style::default().fg(Color::Cyan)),
        ];

    loop {
        while let Ok(event) = rx.try_recv() {
            let color = match event.level.as_str() {
                "ERROR" => Color::Red,
                "STEP" => Color::Yellow,
                "DISCOVERY" => Color::Magenta,
                "RECORD" => Color::Green,
                _ => Color::Cyan,
            };

            let formatted = format!(
                "[{}] [{}] {}",
                event.timestamp.get(11..19).unwrap_or("00:00:00"),
                event.module,
                serde_json::to_string(&event.payload).unwrap_or_default()
            );

            logs.push(ListItem::new(formatted).style(Style::default().fg(color)));

            if logs.len() > 500 {
                logs.remove(0);
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            let header = Paragraph::new(" larp-ops Dashboard | Press 'q' to exit")
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL).title("Status"));

            let list = List::new(logs.clone())
                .block(Block::default().borders(Borders::ALL).title("Event Stream"));

            f.render_widget(header, chunks[0]);
            f.render_widget(list, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
