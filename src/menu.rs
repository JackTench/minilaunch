use std::process::exit;

use inquire::{
    Select,
    Text,
    InquireError
};

use crate::utils;

pub fn main_menu() {
    utils::clear_screen();

    let options: Vec<&str> = vec![
        "Library",
        "Import Steam Games",
        "Quit"
    ];

    let ans = Select::new("minilaunch", options).prompt();

    match ans {
        Ok(choice) => {
            match choice {
                "Import Steam Games" => import_steam_games_menu(),
                "Quit" => {
                    utils::clear_screen();
                    exit(0);
                }
                _ => eprintln!("Invalid Choice!"),
            }
        }
        Err(InquireError::OperationCanceled) => println!("Operation was canceled by the user."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn import_steam_games_menu() {
    utils::clear_screen();

    // Prompt for API key and SteamID.
    let steam_api_key = Text::new("Enter Steam API key:").prompt();
    let steam_id = Text::new("Enter SteamID:").prompt();
}
