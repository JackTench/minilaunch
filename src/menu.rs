use inquire::Select;

use crate::utils;

pub fn main_menu() {
    utils::clear_screen();

    let options: Vec<&str> = vec![
        "Library",
        "Import Steam Games",
        "Quit"
    ];

    let ans = Select::new("minilaunch", options).prompt();
}
