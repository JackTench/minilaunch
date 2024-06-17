use std::io;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::text::Span;
use ratatui::style::{Style, Color};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::layout::{Layout, Constraint, Direction};

use crate::db::Database;

pub fn run_tui(database: &Database) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let games = database.get_all_game_names().unwrap_or_default();
    let mut list_state = ListState::default();
    if !games.is_empty() {
        list_state.select(Some(0));
    }

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

            let items: Vec<ListItem> = games.iter()
                .map(|g| ListItem::new(Span::from(g.clone())))
                .collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Games"))
                .highlight_style(Style::default().bg(Color::LightGreen));

            rect.render_widget(block, chunks[0]);
            rect.render_stateful_widget(list, chunks[1], &mut list_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let Some(selected) = list_state.selected() {
                        let new_selected = if selected > 0 {
                            selected - 1
                        } else {
                            0
                        };
                        list_state.select(Some(new_selected));
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(selected) = list_state.selected() {
                        let new_selected = if selected < games.len() - 1 {
                            selected + 1
                        } else {
                            games.len() - 1
                        };
                        list_state.select(Some(new_selected));
                    }
                }
                KeyCode::Enter => {
                    if let Some(selected) = list_state.selected() {
                        if let Some(game_name) = games.get(selected) {
                            if let Ok((_, game)) = database.fuzzy_search_game(game_name) {
                                database.launch(&game).expect("Failed to launch.");
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    terminal.clear()?;

    Ok(())
}
