mod db;
mod menu;
mod steamapi;
mod utils;

fn main() {
    utils::clear_screen();
    db::setup_database();
    menu::main_menu();
}
