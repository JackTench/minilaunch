use rusqlite::{
    Connection,
    params
};

pub fn setup_database() {
    let conn = Connection::open("minilaunchdata.db").unwrap();
    // Set database schema if first time launch.
    conn.execute_batch(include_str!("sql/schema.sql")).unwrap();
    let _ = conn.close();
}

pub fn add_game(name: String, steamappid: i64) {
    let conn = Connection::open("minilaunchdata.db").unwrap();
    conn.execute(include_str!("sql/add_game.sql"), params![name, steamappid]).unwrap();
    let _ = conn.close();
}
