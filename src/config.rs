use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    version: String,
    modloader: String,
    pub mods: HashMap<String, ModInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ModInfo {
    pub source: String, // placeholder for later
    pub id: String,
    pub required: Option<bool>,
}

pub fn read() -> Result<String, std::io::Error> {
    let config_file = fs::read_to_string("modlist.toml")?;

    Ok(config_file)
}

pub fn parse(
    config_file: String,
) -> Result<(HashMap<String, ModInfo>, String, String), toml::de::Error> {
    let config: Config = toml::from_str(&config_file)?;

    let version = config.version;
    let modloader = config.modloader;

    println!("Your mods:");
    let mods = config.mods;

    Ok((mods, version, modloader))
}

pub fn create_config() -> Result<()> {
    let config = Config {
        version: String::from("26.3"),
        modloader: String::from("fabric"),
        mods: HashMap::from([(
            String::from("sodium"),
            ModInfo {
                source: String::from("modrinth"),
                id: String::from("AANobbMI"),
                required: Some(true),
            },
        )]),
    };

    let toml_string = toml::to_string_pretty(&config)?;

    fs::write("test.toml", toml_string)?;

    Ok(())
}
