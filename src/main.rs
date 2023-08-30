use anyhow::Context;
use clap::Parser;
use directories::ProjectDirs;
use omg_api::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;

mod cli;
use cli::Cli;

#[derive(Deserialize, Serialize)]
struct Config {
    default_name: Option<String>,
    names: HashMap<String, String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Commands::Auth { name } = cli.command {
        let api_key = dialoguer::Password::new()
            .with_prompt("API Key")
            .interact()?;
        save_api_key(&api_key, &name)?;
        println!("API key for address \"{name}\" saved to config.toml");
        std::process::exit(0);
    }

    let config = ProjectDirs::from("com", "supleed2", "omg")
        .and_then(|dirs| read_to_string(dirs.config_dir().join("config.toml")).ok())
        .and_then(|str| toml::from_str::<Config>(&str).ok())
        .context("Unable to parse config.toml as config struct.")?;

    if let Commands::Switch { address } = cli.command {
        match address {
            Some(new) => {
                set_default_name(config, &new)?;
                println!("Default address in Config.toml updated to \"{new}\"")
            }
            None => list_saved_addresses(config)?,
        }
        std::process::exit(0)
    }

    let address = std::env::var("OMGLOL_USERNAME")
        .ok()
        .or(config.default_name)
        .expect("omg.lol address not provided as either environment variable (`OMGLOL_USERNAME`) or in config file as default_name");

    let api_key = std::env::var("OMGLOL_API_KEY")
        .ok()
        .or_else(|| config.names.get(&address).cloned())
        .expect("omg.lol API key not provided as either environment variable (`OMGLOL_API_KEY`) or in config file");

    let resp = cli.command.process(&address, &api_key)?;
    if cli.verbose > 0 {
        println!("\nAPI Response: {resp:?}");
    }

    match resp {
        CommandResponse::Todo(_) => println!(
            "This command has not been implemented yet, please look forward to future releases!"
        ),
        CommandResponse::Address(r) => match r {
            AddressResponse::IsAvailable(IsAvailable { response: r }) => {
                println!(
                    "Address \"{}\" is {}available",
                    r.address,
                    if r.available { "" } else { "NOT " }
                )
            }
            AddressResponse::GetExpiry(GetExpiry { response: r }) => println!("{}", r.message),
            AddressResponse::GetPublicInfo(GetPublicInfo { response: r }) => {
                println!(
                    "Address \"{}\":\n{}\nExpired: {}\nVerified: {}",
                    r.address, r.message, r.expiration.expired, r.verification.verified
                )
            }
            AddressResponse::GetInfo(GetInfo { response: r }) => {
                println!(
                    "Address \"{}\" (owned by \"{}\"):\n{}\nExpired: {}\nVerified: {}",
                    r.address, r.owner, r.message, r.expiration.expired, r.verification.verified
                )
            }
        },
    }

    anyhow::Ok(())
}

fn list_saved_addresses(config: Config) -> anyhow::Result<()> {
    println!("Saved addresses:");
    for name in config.names.keys() {
        println!("{name}");
    }
    let exe = std::env::current_exe()?;
    let exe = exe.file_name().and_then(|e| e.to_str()).unwrap_or("omg");
    if let Some(name) = config.default_name {
        println!("Current default: {name}, use `{exe} switch [name]` to change");
    } else {
        println!("Current default unset, use `{exe} switch [name]` to set");
    }
    anyhow::Ok(())
}

fn set_default_name(config: Config, new: &str) -> std::io::Result<()> {
    if config.names.contains_key(new) {
        let config_path = ProjectDirs::from("com", "supleed2", "omg")
            .expect("Unable to access app config directory (while setting default address).")
            .config_dir()
            .join("config.toml");
        let toml_str = toml::to_string_pretty(&Config {
            default_name: Some(new.to_string()),
            names: config.names,
        })
        .expect("Unable to convert updated config to TOML (while setting default address).");
        std::fs::write(config_path, toml_str)
    } else {
        eprintln!("Address \"{new}\" not in config.toml saved addresses");
        std::process::exit(1)
    }
}

fn save_api_key(api_key: &str, name: &str) -> std::io::Result<()> {
    let config_path = ProjectDirs::from("com", "supleed2", "omg")
        .expect("Unable to access app config directory (while saving API key).")
        .config_dir()
        .join("config.toml");
    let _ = std::fs::create_dir_all(
        config_path
            .parent()
            .expect("Unable to get parent dir of config.toml"),
    );
    let Config {
        default_name,
        mut names,
    } = read_to_string(&config_path)
        .ok()
        .and_then(|str| toml::from_str::<Config>(&str).ok())
        .unwrap_or_else(|| {
            eprintln!("Failed to load config, recreating... (Old config will be overwritten)");
            Config {
                default_name: Some(name.to_string()),
                names: HashMap::new(),
            }
        });
    let default_name = default_name.or_else(|| {
        eprintln!("No default_name in config, setting to \"{name}\"");
        Some(name.to_string())
    });
    names.insert(name.to_string(), api_key.to_string());
    let toml_str = toml::to_string_pretty(&Config {
        default_name,
        names,
    })
    .expect("Unable to convert updated config to TOML (while saving API key).");
    std::fs::write(&config_path, toml_str)
}

// Clap arg relations: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#argument-relations
// Clap derive docs: https://docs.rs/clap/latest/clap/_derive/index.html
// Thiserror docs: https://docs.rs/thiserror/latest/thiserror/index.html
