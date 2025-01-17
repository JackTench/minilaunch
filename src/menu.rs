use std::process::exit;

use inquire::{
    Confirm,
    Select,
    Text,
    InquireError
};

use crate::db;
use crate::steam;
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
                "Library" => library_menu(),
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

fn library_menu() {
    utils::clear_screen();

    // Get games from db.
    let games = db::get_games();
    let game_names: Vec<String> = games.iter()
        .map(|game| game.name.clone())
        .collect();

    // Don't crash if there's no games in the database.
    if game_names.len() == 0 {
        println!("There are no games in the database!");
        exit(0);
    }

    let ans = Select::new("Games", game_names)
        .with_page_size((utils::get_terminal_height() - 2) as usize)
        .prompt()
        .expect("Failed to select a game.");

    let selected_game = games.iter()
        .find(|game| game.name == ans)
        .expect("Selected game not found in the database.");

    steam::launch(&selected_game);
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
