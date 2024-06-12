mod db;

use db::Database;

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

    let args = CliArgs::parse();

    if args.command == "run".to_string() {
        let game_names = database.get_all_game_names().unwrap();
        let game_string = args.game.to_string();
        if game_names.iter().any(|e| game_string.contains(e)) {
            println!("game is in db");
        } else {
            println!("game not in db");
        }
    }
}
