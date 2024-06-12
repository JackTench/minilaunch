use rusqlite::{Connection, Result, params};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct Game {
    pub id: i32,
    pub name: String,
    pub platform: String,
    pub launch_cmd: String,
    pub play_count: i32,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    
    pub fn new() -> Result<Self> {
        let conn = Connection::open("minilaunchdata.db")?;
        Ok(Database { conn })
    }

    pub fn setup(&self) {
        self.conn.execute_batch(include_str!("sql/schema.sql")).unwrap();
    }

    pub fn get_all_games(&self) -> Result<Vec<Game>> {
        let mut statement = self.conn.prepare(include_str!("sql/get_all_games.sql"))?;
        let rows = statement.query_map([], |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
                platform: row.get(2)?,
                launch_cmd: row.get(3)?,
                play_count: row.get(4)?,
            })
        })?;
        rows.collect()
    }

    pub fn get_all_game_names(&self) -> Result<Vec<String>> {
        let mut statement = self.conn.prepare(include_str!("sql/get_all_games.sql"))?;
        let rows = statement.query_map([], |row| row.get(1))?;
        rows.collect()
    }

    pub fn fuzzy_search_game(&self, query: &str) -> std::result::Result<(String, Game), String> {
        let matcher = SkimMatcherV2::default();
        let game_names = self.get_all_game_names().map_err(|e| e.to_string())?;
        let mut best_match = None;
        let mut best_score = 0;

        for game_name in &game_names {
            if let Some(score) = matcher.fuzzy_match(game_name, query) {
                if score > best_score {
                    best_score = score;
                    best_match = Some(game_name.clone());
                }
            }
        }

        match best_match {
            Some(best_match) => {
                let mut statement = self.conn.prepare("SELECT * FROM games WHERE name = ?1").map_err(|e| e.to_string())?;
                let game = statement.query_row(params![best_match.clone()], |row| {
                    Ok(Game {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        platform: row.get(2)?,
                        launch_cmd: row.get(3)?,
                        play_count: row.get(4)?,
                    })
                }).map_err(|e| e.to_string())?;
                Ok((best_match, game))
            }
            None => Err("No matching game found".to_string())
        }
    }

    pub fn new_game(&self, name: &str, platform: &str, launch_cmd: &str) {
        self.conn.execute(include_str!("sql/new_game.sql"), params![name, platform, launch_cmd, 0]).unwrap();
    }

}
