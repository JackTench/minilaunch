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

pub fn yn_prompt_bool(message: &str) -> bool {
    // Format prompt message.
    let mut prompt_message = String::new();
    prompt_message.push_str(message);
    prompt_message.push_str(" (y/n) ");
    print!("{}", prompt_message);

    // Read input.
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    // Parse
    // TODO: Investigate if doing this with recursion is okay...
    match input.trim().to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("Invalid input. Please enter Y or N.");
            yn_prompt_bool(message)
        }
    }
}
