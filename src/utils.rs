use std::io::stdout;

use crossterm::{
    execute,
    terminal::{
        Clear,
        ClearType,
        size
    },
    cursor::MoveTo
};

pub fn clear_screen() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();
}

pub fn get_terminal_height() -> u16 {
    let (_, height) = size().unwrap();
    height
}
