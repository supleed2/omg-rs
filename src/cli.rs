use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Set which omg.lol username to use, overrides config and environment variable (OMGLOL_USERNAME)
    #[clap(short, long)]
    pub name: Option<String>,
    /// Which subciomnmnandnjhsdflk
    #[clap(subcommand)]
    pub command: Option<Commands>,
    /// Print debug information, repeat for higher levels of debug info (max 1)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Subcommand)]
pub enum Commands {
    Account{ // TODO: include or not?
        /// Email of your omg.lol account, needed for Account commands only
        email: String,
        #[clap(subcommand)]
        command: Account,
    },
    Address,
    Auth {
        api_key: String,
    },
    Directory,
    /// Adjust the switchboard / DNS records for your omg.lol subdomain
    DNS,
    Email,
    NowPage,
    Pastebin,
    Preferences{ // TODO: include or not?
    },
    PURL,
    Service{ // TODO: include or not?
    },
    Status, 
    Theme{ // TODO: include or not?
    },
    Web,
    Weblog {
        #[arg(short, long)]
        yay: u8,

        #[arg(short, long)]
        nay: Option<u8>,
    },
}

// Link for info to add above: https://api.omg.lol/

#[derive(Subcommand)]
pub enum Account {
    /// Get information about your account
    GetInfo,
    GetAddrs,
    GetName,
    SetName {
        name: String,
    },
    GetSessions,
    RemoveSession {
        session_id: String,
    },
    GetSettings,
    SetSettings {
        json_data: String,
    },
}

#[derive(Subcommand)]
pub enum Address {}

#[derive(Subcommand)]
pub enum Auth {}

#[derive(Subcommand)]
pub enum Directory {}

#[derive(Subcommand)]
pub enum DNS {}

#[derive(Subcommand)]
pub enum Email {}

#[derive(Subcommand)]
pub enum NowPage {}

#[derive(Subcommand)]
pub enum Pastebin {}

#[derive(Subcommand)]
pub enum Preferences {}

#[derive(Subcommand)]
pub enum PURL {}

#[derive(Subcommand)]
pub enum Service {}

#[derive(Subcommand)]
pub enum Status  {}

#[derive(Subcommand)]
pub enum Theme {}

#[derive(Subcommand)]
pub enum Web {}

#[derive(Subcommand)]
pub enum Weblog {}