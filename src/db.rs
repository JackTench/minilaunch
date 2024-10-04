use rusqlite::Connection;

pub fn setup_database() {
    let conn = Connection::open("minilaunchdata.db").unwrap();
    let _ = conn.close();
}
