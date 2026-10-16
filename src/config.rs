use serde::Deserialize;

use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    version: String,
    modloader: String,
    pub mods: HashMap<String, Mod>,
}

#[derive(Deserialize)]
pub struct Mod {
    pub source: String, // placeholder for later
    pub id: String,
}

pub fn read() -> Result<String, std::io::Error> {
    let config_file = fs::read_to_string("modlist.toml")?;

    Ok(config_file)
}

pub fn parse(
    config_file: String,
) -> Result<(HashMap<String, Mod>, String, String), toml::de::Error> {
    let config: Config = toml::from_str(&config_file)?;

    let version = config.version;
    let modloader = config.modloader;

    println!("Your mods:");
    let mods = config.mods;

    Ok((mods, version, modloader))
}

pub fn create_config() {}
