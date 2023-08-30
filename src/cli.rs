use clap::Parser;
use omg_api::Commands;

#[derive(Parser)]
#[command(name = "omg-rs", version,
    long_about = concat!("\x1b[38;5;205m", include_str!("prami_sjw.txt"),
    "\n\nA cli client for omg.lol, written in Rust 🦀\x1b[0m"))]
/// A cli client for omg.lol, written in Rust 🦀
pub struct Cli {
    /// Categories of commands to interact with the omg.lol API
    #[clap(subcommand)]
    pub command: Commands,
    /// Print debug information, repeat for higher levels of debug info (max 1)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
