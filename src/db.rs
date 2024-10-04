use rusqlite::Connection;

pub fn setup_database() {
    let conn = Connection::open("minilaunchdata.db").unwrap();
    // Set database schema if first time launch.
    conn.execute_batch(include_str!("sql/schema.sql")).unwrap();
    let _ = conn.close();
}
