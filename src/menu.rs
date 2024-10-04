use std::process::exit;

use inquire::{
    Select,
    InquireError
};

use crate::utils;

pub fn main_menu() {
    utils::clear_screen();

    let options: Vec<&str> = vec![
        "Library",
        "Import",
        "Quit",
    ];

    let ans = Select::new("minilaunch", options).prompt();

    match ans {
        Ok(choice) => {
            match choice {
                "Quit" => {
                    utils::clear_screen();
                    exit(0);
                }
                _ => eprintln!("Invalid choice!"),
            }
        }
        Err(InquireError::OperationCanceled) => println!("Operation was canceled by the user."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
