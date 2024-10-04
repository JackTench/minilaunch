use rusqlite::{
    Connection,
    params,
    Result
};

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("minilaunchdata.db")?;
        conn.execute_batch(include_str!("sql/schema.sql"))?;
        Ok(Database { conn })
    }

    pub fn add_game(&self, name: String, steamappid: i64) {
        self.conn.execute(include_str!("sql/add_game.sql"), params![name, steamappid]).unwrap();
    }
}

pub struct Game {
    pub id: i64,
    pub name: String,
    pub steamappid: i64
}
