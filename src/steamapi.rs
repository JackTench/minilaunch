use std::error::Error;

use reqwest::blocking::Client;
use serde_json::Value;

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
        for game in games {
            let name = game.get("name").unwrap();
            let steam_app_id = game.get("appid").unwrap().to_string();
            println!("{} {}", name, steam_app_id);
        }
    }

    Ok(())
}
