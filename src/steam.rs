use std::env;

use dotenv::dotenv;

pub fn import_steam_games() {
    // Load Steam API key environment variable.
    dotenv().ok();
    let api_key = env::var("STEAM_API_KEY").expect("Could not find Steam API key environment variable.");
}
