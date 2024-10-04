mod db;
mod menu;
mod utils;

fn main() {
    let _db = db::Database::new();

    utils::clear_screen();
    menu::main_menu();
}
