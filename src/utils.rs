use std::io::stdout;

use crossterm::{
    execute,
    terminal::{
        Clear,
        ClearType
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
