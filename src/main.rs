mod db;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    command: String,
    game: String,
}

fn main() {
    let args = CliArgs::parse();

    if args.command == "run".to_string() {
        println!("running {}", args.game)
    }
}
