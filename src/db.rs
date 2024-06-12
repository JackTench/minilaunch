use rusqlite::{Connection, Result, params};

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
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn new_game(&self, name: &str, platform: &str, launch_cmd: &str) {
        let name_string: String = name.to_string();
        let platform_string: String = platform.to_string();
        let launch_cmd_string: String = launch_cmd.to_string();
        self.conn.execute(include_str!("sql/new_game.sql"), params![name_string, platform_string, launch_cmd_string, 0]).unwrap();
    }

}
