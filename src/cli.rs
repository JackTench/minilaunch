use std::io::{self, Write};

use clap::Parser;

use crate::db::Database;
use crate::tui::run_tui;
use crate::steam::import_steam_games;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    pub command: Option<String>,
    pub game: Option<String>,
}

impl CliArgs {
    pub fn parse_args() -> Self {
        CliArgs::parse()
    }

    pub fn handle_command(self, database: &Database) {
        // Launch TUI when no CLI arguments are given.
        if self.command.is_none() {
            run_tui(database).expect("Failed to launch TUI.");
            return;
        }

        match self.command.as_deref() {
            // Run game from CLI.
            Some("run") => {
                if let Some(game_string) = self.game {
                    match database.fuzzy_search_game(&game_string) {
                        Ok((game_name, game)) => {
                            if game_name == game_string {
                                database.launch(&game).expect("Failed to launch.");
                            } else {
                                if yn_prompt_bool(format!("Did you mean {}?", game_name).as_str()) {
                                    database.launch(&game).expect("Failed to launch.");
                                } else {
                                    println!("Game not found.");
                                }
                            }
                        }
                        Err(err) => println!("{}", err),
                    }
                } else {
                    println!("No game specified.");
                }
            }
            Some("add") => {
                if let Some(game_name) = self.game {
                    let platform = inline_prompt_str("Platform: ");
                    let launch_cmd = inline_prompt_str("Launch command: ");
                    database.new_game(&game_name, &platform, &launch_cmd);
                    println!("Game has been added.");
                } else {
                    println!("No game specified.");
                }
            }
            Some("steam") => {
                let id = inline_prompt_str("Steam ID: ");
                let _ = import_steam_games(&id, &database);
                println!("Steam library has been imported.");
            }
            _ => {
                println!("Unknown command.");
            }
        }
    }
}

pub fn inline_prompt_str(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn yn_prompt_bool(message: &str) -> bool {
    // Format prompt message.
    let mut prompt_message = String::new();
    prompt_message.push_str(message);
    prompt_message.push_str(" (y/n) ");
    print!("{}", prompt_message);

    // Read input.
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    // Parse
    // TODO: Investigate if doing this with recursion is okay...
    match input.trim().to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("Invalid input. Please enter Y or N.");
            yn_prompt_bool(message)
        }
    }
}
