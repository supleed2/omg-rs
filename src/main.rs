use anyhow::Context;
use clap::Parser;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

mod cli;
use cli::{Cli, Commands};

#[derive(Default, Deserialize, Serialize)]
struct Config {
    api_key: Option<String>,
    name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Auth { api_key }) = cli.command {
        match save_api_key(&api_key) {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
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

    let name = cli.name
        .or(std::env::var("OMGLOL_USERNAME").ok())
        .or(config.name)
        .expect("omg.lol username not provided as command line option, environment variable or in config file");

    println!("omg-rs, ready for @{name}");
    if cli.verbose > 0 {
        println!("API key: {}", api_key);
    }
    Ok(())
}

fn save_api_key(api_key: &str) -> std::io::Result<()> {
    let config_path = ProjectDirs::from("com", "supleed2", "omg")
        .expect("Unable to access app config directory (while saving API key).")
        .config_dir()
        .join("config.toml");
    let _ = std::fs::create_dir_all(
        &config_path
            .parent()
            .expect("Unable to get parent dir of config.toml"),
    );
    let Config { api_key: _, name } = read_to_string(&config_path)
        .ok()
        .and_then(|str| toml::from_str::<Config>(&str).ok())
        .unwrap_or_default();
    let toml_str = toml::to_string_pretty(&Config {
        api_key: Some(api_key.to_string()),
        name,
    })
    .expect("Unable to convert updated config to TOML (when trying to save API key).");
    std::fs::write(&config_path, toml_str)
}

// Tutorial at: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
// More info at: https://docs.rs/clap/latest/clap/_derive/index.html
