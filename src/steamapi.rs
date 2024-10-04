use std::error::Error;

use reqwest::blocking::Client;

pub fn import_steam_games(steam_api_key: &str, steam_id: &str) -> Result<(), Box<dyn Error>> {
    // Download JSON of steam games for given SteamID.
    let client = Client::new();
    let query_url = format!(
        "http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json&include_appinfo=true",
        steam_api_key, steam_id
    );
    let response = client.get(&query_url).send()?.text()?;
    println!("{}", response);

    Ok(())
}
