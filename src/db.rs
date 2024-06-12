use rusqlite::{Connection, Result};

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

}
