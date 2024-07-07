use std::env;
use std::error::Error;

use dotenv::dotenv;
use reqwest::blocking::Client;

pub fn import_steam_games(steam_id: &str) -> Result<(), Box<dyn Error>> {
    // Load Steam API key environment variable.
    dotenv().ok();
    let api_key = env::var("STEAM_API_KEY").expect("Could not find Steam API key environment variable.");

    // Download JSON of steam games for given Steam ID.
    let client = Client::new();
    let query_url = format!(
        "http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json&include_appinfo=true",
        api_key, steam_id
    );

    let response = client.get(&query_url).send()?.text()?;

    Ok(())
}
