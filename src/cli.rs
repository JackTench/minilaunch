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
