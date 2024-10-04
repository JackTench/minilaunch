use rusqlite::{
    Connection,
    Result
};

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("minilaunchdata.db")?;
        Ok(Database { conn })
    }
}
