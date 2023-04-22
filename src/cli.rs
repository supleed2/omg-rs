use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Set a custom username
    #[clap(short, long)]
    pub username: Option<String>,

    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Subcommand)]
pub enum Commands {
    Auth {
        api_key: String,
    },
    Weblog {
        #[arg(short, long)]
        yay: u8,

        #[arg(short, long)]
        nay: Option<u8>,
    },
    Status {
        #[arg(long)]
        message: String,

        /// Does this explain it?
        req: String,
    },
}