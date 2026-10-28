use anyhow::{Context, Result, bail};
use inquire::Confirm;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

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

const CONFIG_PATH: &str = "modlist.toml";

pub fn read() -> Result<String, std::io::Error> {
    fs::read_to_string(CONFIG_PATH)
}

pub fn parse(config_file: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(config_file)
}

pub fn list(c: &Config) {
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

    fs::write(CONFIG_PATH, toml_string)?;

    Ok(())
}

//loads the config to stop redundancy with the list and update subcommands
pub fn load() -> Result<Config> {
    let contents: String = match read() {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            if !prompt_create()? {
                bail!("no {CONFIG_PATH} provided and none was created");
            }

            println!("config generated at {CONFIG_PATH}, go edit it!");
            std::process::exit(0);
        }
        Err(e) => return Err(e).with_context(|| format!("failed to read {CONFIG_PATH}")),
    };

    parse(&contents).with_context(|| format!("failed to parse {CONFIG_PATH}"))
}

//also i dont think we really have to follow the like no printing allowed outside of main idk its dumb and doesnt apply to us
pub fn prompt_create() -> Result<bool> {
    let create = Confirm::new("No manifest found in cwd, would you like to create one?")
        .with_default(true)
        .prompt()?;
    if create {
        println!("Creating default configuration...");
        create_config()?;
    }
    Ok(create)
}
