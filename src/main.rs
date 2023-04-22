use anyhow::Context;
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Default, Deserialize, Serialize)]
struct Config {
    api_key: Option<String>,
    username: Option<String>,
}

#[derive(Parser)]
struct Cli {
    /// Set a custom username
    #[clap(short, long)]
    username: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Auth { api_key }) = cli.command {
        let config_path = ProjectDirs::from("com", "supleed2", "omg")
            .context("Unable to access app config directory (while saving API key).")?
            .config_dir().join("config.toml");
        let _ = std::fs::create_dir_all(&config_path.parent().expect("Unable to get parent dir of config.toml"));
        let Config { api_key: _, username } = read_to_string(&config_path).ok()
            .and_then(|str| toml::from_str::<Config>(&str).ok())
            .unwrap_or_default();
        let toml_str = toml::to_string_pretty(&Config {
            api_key: Some(api_key),
            username,
        })
        .expect("Unable to convert updated config to TOML (when trying to save API key).");
        match std::fs::write(&config_path, toml_str) {
            Ok(_) => {
                println!("Saved API key to config file: {}", config_path.display());
                std::process::exit(0);
            }
            Err(_) => {
                eprintln!(
                    "Failed to save API key to config file: {}",
                    config_path.display()
                );
                std::process::exit(1);
            }
        };
    }

    let config = ProjectDirs::from("com", "supleed2", "omg")
        .and_then(|dirs| read_to_string(dirs.config_dir().join("config.toml")).ok())
        .and_then(|str| toml::from_str::<Config>(&str).ok())
        .context("Unable to parse config.json as config struct.")?;

    let api_key = std::env::var("OMGLOL_API_KEY")
        .ok()
        .or(config.api_key)
        .expect("omg.lol API key not provided as either environment variable or in config file");

    let username = cli.username
        .or(std::env::var("OMGLOL_USERNAME").ok())
        .or(config.username)
        .expect("omg.lol username not provided as command line option, environment variable or in config file");

    println!("omg-rs, ready for @{username}");
    println!("API key: {}", api_key);
    Ok(())
}

// Tutorial at: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
// More info at: https://docs.rs/clap/latest/clap/_derive/index.html
