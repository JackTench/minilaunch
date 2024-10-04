use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{
        Clear,
        ClearType
    }
};

pub fn clear_screen() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();
}
