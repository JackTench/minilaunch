mod db;

use db::Database;

use std::io::{self, Write};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    command: String,
    game: String,
}

fn main() {
    let database = Database::new().unwrap();
    database.setup();

    // Code for testing.
    database.new_game("minecraft", "PC", "prismlauncher");
    database.new_game("oot", "N64", "soh");

    let args = CliArgs::parse();

    if args.command == "run".to_string() {
        let game_string = args.game.to_string();
        match database.fuzzy_search_game(&game_string) {
            Ok((game_name, game)) => {
                println!("Do you mean {}? (y/n)", game_name);
                let mut input = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().to_lowercase() == "y" {
                    println!("Launching game: {}", game.name);
                } else {
                    println!("Game not found.");
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
