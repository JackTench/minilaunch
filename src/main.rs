mod db;

use db::Database;

use std::io::{self, Write};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    command: String,
    game: Option<String>,
}

fn main() {
    let database = Database::new().unwrap();
    database.setup();

    let args = CliArgs::parse();

    match args.command.as_str() {
        "run" => {
            if let Some(game_string) = args.game {
                match database.fuzzy_search_game(&game_string) {
                    Ok((game_name, game)) => {
                        if game_name == game_string {
                            println!("Launching Game: {}", game_name);
                        } else {
                            print!("Did you mean {}? (y/n) ", game_name);
                            let mut input = String::new();
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            if input.trim().to_lowercase() == "y" {
                                println!("Launching Game: {}", game.name);
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
        "add" => {
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
