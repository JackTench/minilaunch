use std::process::{
    Command,
    Stdio
};

use crate::db::Game;

pub fn launch(game: &Game) {
    let launch_url = format!("steam://run/{}", game.steamappid);

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", &launch_url])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to launch.");
    }

    #[cfg(target_os  = "linux")]
    {
        Command::new("xdg-open")
            .arg(&launch_url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to launch.");
    }
}
