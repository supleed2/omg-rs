use crate::commands::Commands;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Set which omg.lol username to use, overrides config and environment variable (OMGLOL_USERNAME)
    #[clap(short, long)]
    pub name: Option<String>,
    /// Categories of commands to interact with the omg.lol API
    #[clap(subcommand)]
    pub command: Option<Commands>,
    /// Print debug information, repeat for higher levels of debug info (max 1)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
