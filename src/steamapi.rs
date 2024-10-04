use std::error::Error;

use reqwest::blocking::Client;
use serde_json::Value;
use indicatif::ProgressBar;

use crate::db;

pub fn import_steam_games(steam_api_key: &str, steam_id: &str) -> Result<(), Box<dyn Error>> {
    // Download JSON of steam games for given SteamID.
    let client = Client::new();
    let query_url = format!(
        "http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json&include_appinfo=true",
        steam_api_key, steam_id
    );
    let response = client.get(&query_url).send()?.text()?;
    //println!("{}", response);

    // Parse JSON response.
    let json: Value = serde_json::from_str(&response).unwrap();
    if let Some(games) = json.pointer("/response/games").and_then(|v| v.as_array()) {
        // Create progress bar.
        let bar = ProgressBar::new(games.len() as u64);

        // Add games to db.
        for game in games {
            let name = game.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Game").to_string();
            let steam_app_id = game.get("appid").and_then(|v| v.as_i64()).unwrap_or(-1);
            db::add_game(name, steam_app_id);
            // Tick progress bar.
            bar.inc(1);
        }

        bar.finish();
    }

    Ok(())
}
