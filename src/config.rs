use serde::Deserialize;
use std::{env, fs};
use toml::Value;

#[derive(Deserialize)]
pub struct Config {
    pub use_emoji: bool,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Instalation path
    let mut installation_path = env::current_exe().expect("Unable to get current path");
    installation_path.pop();
    installation_path.push("devly.toml");

    // Reading devly.toml file
    let mut config = load_config_from_file(&installation_path.to_str().unwrap())?;

    // Reading current config file
    if let Ok(current_dir_config) = load_config_from_file("devly.toml") {
        config = Config {
            ..current_dir_config
        }
    }

    Ok(config)
}

fn load_config_from_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {

    let content = fs::read_to_string(path)?;
    let data: Value = content.parse()?;
    let config_value = data.get("Config").ok_or(format!("Missing [Config] section on: {}", {path}))?;
    let config = Config::deserialize(config_value.clone())?;

    Ok(config)
}