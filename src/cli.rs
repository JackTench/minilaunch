use std::io::{self, Write};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    pub command: Option<String>,
    pub game: Option<String>,
}

impl CliArgs {
    pub fn parse_args() -> Self {
        CliArgs::parse()
    }
}

pub fn inline_prompt_str(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
