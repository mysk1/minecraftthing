use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: String,
    pub modloader: String,
    pub threshold: Option<f64>,
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

pub fn parse(config_file: String) -> Result<Config, toml::de::Error> {
    let config: Config = toml::from_str(&config_file)?;

    Ok(config)
}

pub fn list(c: Config) {
    let Config {
        modloader,
        mods,
        version,
        threshold,
    } = c;

    println!("Modloader: {modloader}");
    if let Some(t) = threshold {
        println!("Threshold: {t}");
    }
    println!("Your mods:");
    for (name, _info) in mods {
        println!("name: {name}, version: {version}");
    }
}

pub fn create_config() -> Result<()> {
    let config = Config {
        version: String::from("26.3"),
        modloader: String::from("fabric"),
        threshold: Some(0.9),
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
