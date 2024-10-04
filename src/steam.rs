use std::process::{
    Command,
    Stdio
};

use crate::db::Game;

pub fn launch(game: &Game) {
    let launch_cmd = format!("steam -applaunch {} &", game.steamappid);

    Command::new("sh")
        .arg("-c")
        .arg(launch_cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to launch.");
}
