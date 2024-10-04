use std::process::exit;

use inquire::{
    Confirm,
    Select,
    Text,
    InquireError
};

use crate::steamapi;
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
    let steam_api_key = Text::new("Enter Steam API key:").prompt().unwrap();
    let steam_id = Text::new("Enter SteamID:")
        .with_help_message("In numeric form, sometimes called SteamID64.")
        .prompt().unwrap();

    steamapi::import_steam_games(&steam_api_key, &steam_id).unwrap();

    // Prompt for returning to main menu.
    let ret = Confirm::new("Would you like to return to the main menu?")
        .with_default(true)
        .with_help_message("If no, minilaunch will close.")
        .prompt();

    match ret {
        Ok(true) => main_menu(),
        Ok(false) => {
            utils::clear_screen();
            exit(0);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
