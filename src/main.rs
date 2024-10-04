mod db;
mod menu;
mod utils;

fn main() {
    let data = db::Database::new().unwrap();

    utils::clear_screen();
    menu::main_menu();
}
