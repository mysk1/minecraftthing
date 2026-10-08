use std::fs;
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    version: String,
    modloader: String,
    mods: HashMap<String, Mod>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Mod {
    pub source: String,
    pub id: String,
}

pub fn parse() -> () {
    let config_file = fs::read_to_string("modlist.toml").expect("Shoulda had a file buddy");

    let config: Config = toml::from_str(&config_file).unwrap();


    println!("Your mods:");
    let mods = config.mods;
    let mut index = 1;
    for (name, info) in mods {
        let Mod { id, source} = info;

        println!("{index}. Name: {name:?}, id: {id}, source: {source}");
        index += 1;
    }
}
