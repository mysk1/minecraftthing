use serde::Deserialize;
use std::fs;

use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    version: String,
    modloader: String,
    pub mods: HashMap<String, Mod>,
}

#[derive(Deserialize)]
pub struct Mod {
    pub source: String,
    pub id: String,
}

pub fn parse() -> (HashMap<String, Mod>, String, String) {
    let config_file = fs::read_to_string("modlist.toml").expect("Shoulda had a file buddy");

    let config: Config = toml::from_str(&config_file).unwrap();

    let version = config.version;
    let modloader = config.modloader;

    println!("Your mods:");
    let mods = config.mods;

    return (mods, version, modloader);
}
