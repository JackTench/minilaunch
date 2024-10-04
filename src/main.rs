mod db;
mod utils;

fn main() {
    utils::clear_screen();
    db::setup_database();
}
