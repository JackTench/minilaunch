use std::env;
use std::error::Error;

use dotenv::dotenv;
use reqwest::blocking::Client;
use serde_json::Value;

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
    let json: Value = serde_json::from_str(&response)?;

    if let Some(games) = json.pointer("/response/games").and_then(|v| v.as_array()) {
        for game in games {
            let appid = game.get("appid").and_then(|v| v.as_i64()).unwrap_or(-1) as i32;
            let name = game.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Game").to_string();
        }
    }

    Ok(())
}
