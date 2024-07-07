mod db;
mod tui;
mod steam;

use db::Database;
use tui::run_tui;

use std::io::{self, Write};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    command: Option<String>,
    game: Option<String>,
}

fn main() {
    let database = Database::new().unwrap();
    database.setup();

    let args = CliArgs::parse();

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
                let platform = prompt("Platform: ");
                let launch_cmd = prompt("Launch command: ");
                database.new_game(&game_name, &platform, &launch_cmd);
                println!("Game has been added.");
            } else {
                println!("No game name specified.");
            }
        }
        _ => {
            println!("Unknown command.");
        }
    }
}

fn prompt(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
