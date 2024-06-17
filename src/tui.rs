use std::io;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::text::Span;
use ratatui::style::{Style, Color};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::layout::{Layout, Constraint, Direction};

use crate::db::Database;

pub fn run_tui(database: &Database) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ].as_ref()
                )
                .split(size);

            let block = Block::default()
                .borders(Borders::ALL)
                .title("minilaunch");

            let games = database.get_all_game_names().unwrap_or_default();
            let items: Vec<ListItem> = games.iter()
                .map(|g| ListItem::new(Span::from(g.clone())))
                .collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Games"))
                .highlight_style(Style::default().bg(Color::LightGreen));

            rect.render_widget(block, chunks[0]);
            rect.render_widget(list, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.clear()?;
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
