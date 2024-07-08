mod cli;
mod db;
mod tui;
mod steam;

use cli::CliArgs;
use cli::inline_prompt_str;
use db::Database;
use tui::run_tui;
use steam::import_steam_games;

use std::io::{self, Write};

fn main() {
    let database = Database::new().unwrap();
    database.setup();

    let args = CliArgs::parse_args();

    if args.command.is_none() {
        run_tui(&database).expect("Failed to launch TUI.");
        return;
    }

    match args.command.as_deref() {
        Some("run") => {
            if let Some(game_string) = args.game {
                match database.fuzzy_search_game(&game_string) {
                    Ok((game_name, game)) => {
                        if game_name == game_string {
                            database.launch(&game).expect("Failed to launch.");
                        } else {
                            print!("Did you mean {}? (y/n) ", game_name);
                            let mut input = String::new();
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            if input.trim().to_lowercase() == "y" {
                                database.launch(&game).expect("Failed to launch");
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
            if let Some(game_name) = args.game {
                let platform = inline_prompt_str("Platform: ");
                let launch_cmd = inline_prompt_str("Launch command: ");
                database.new_game(&game_name, &platform, &launch_cmd);
                println!("Game has been added.");
            } else {
                println!("No game name specified.");
            }
        }
        Some("steam") => {
            let id = inline_prompt_str("Steam ID:");
            let _ = import_steam_games(&id, &database);
            println!("Steam library has been imported.");
        }
        _ => {
            println!("Unknown command.");
        }
    }
}
