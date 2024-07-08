mod cli;
mod db;
mod tui;
mod steam;

use cli::CliArgs;
use db::Database;

fn main() {
    let database = Database::new().unwrap();
    database.setup();

    let args = CliArgs::parse_args();
    args.handle_command(&database);
}
