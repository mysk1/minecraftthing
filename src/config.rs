use std::fs;
use toml::Table;

pub struct Mod {
    game_version: String,
    loader: String,
    slug: String,
    projectid: String,
}

pub fn parse() -> () {
    let config_file = fs::read_to_string("modlist.toml").expect("Shoulda had a file buddy");

    let parsed_config = config_file.parse::<Table>().unwrap();

    if let Some(mods) = parsed_config.get("mods") {
        println!("Mod list = {mods}");
    }
}
