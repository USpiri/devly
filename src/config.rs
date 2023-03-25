use serde::Deserialize;
use std::{env, fs};
use toml::Value;

#[derive(Deserialize)]
pub struct Config {
    pub use_emoji: bool,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Instalation path
    let mut current_exe = env::current_exe().expect("No se puede obtener la ruta actual");
    current_exe.pop();
    current_exe.push("devly_commits.toml");

    // Reading devly_commits.toml file
    let content = fs::read_to_string(current_exe)?;
    let data: Value = content.parse()?;
    let config_value = data.get("Config").ok_or("Missing [Config] section")?;
    let config = Config::deserialize(config_value.clone())?;

    Ok(config)
}
