use std::io;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::db::Database;

pub fn run_tui(database: &Database) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    Ok(())
}
