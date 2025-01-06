use std::fs::create_dir_all;
use std::path::PathBuf;

use rusqlite::{
    Connection,
    params
};
use dirs::data_local_dir;

pub struct Platform {
    pub name: String,
    pub launch_prefix: String,
}

pub struct Game {
    pub name: String,
    pub steamappid: i64
}

fn database_file() -> PathBuf {
    let mut path = data_local_dir().expect("Failed to find the user's local data directory.");
    path.push("minilaunch");
    create_dir_all(&path).expect("Failed to create minilaunch folder.");
    path.push("minilaunchdata.db");
    path
}

pub fn setup_database() {
    let conn = Connection::open(database_file()).unwrap();
    // Set database schema if first time launch.
    conn.execute_batch(include_str!("sql/schema.sql")).unwrap();
    let _ = conn.close();
}

pub fn add_game(name: String, steamappid: i64) {
    let conn = Connection::open(database_file()).unwrap();
    conn.execute(include_str!("sql/add_game.sql"), params![name, steamappid]).unwrap();
    let _ = conn.close();
}

pub fn get_games() -> Vec<Game> {
    let conn = Connection::open(database_file()).unwrap();

    // Query database.
    let mut statement = conn.prepare(include_str!("sql/get_games.sql")).map_err(|e| e.to_string()).unwrap();

    // Fetch rows.
    let games_iter = statement.query_map(params![], |row| {
        Ok(Game {
            name: row.get(1)?,
            steamappid: row.get(2)?,
        })
    }).unwrap();

    // Collect into vector.
    let games: Vec<Game> = games_iter
        .map(|game| game.unwrap())
        .collect();

    games
}
