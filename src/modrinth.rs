use anyhow::Result;
use std::io::{Read};
use serde::{Serialize};
use reqwest::blocking::Client;
use std::collections::HashMap;

use crate::config::Mod;

pub fn create_client() -> Client {
    let client = Client::new();
    return client;
}

pub fn call_modrinth(mods: HashMap<String, Mod>, version: String, loader: String, client: Client) -> Result<()> {
    for (name, info) in mods {
        let Mod { id, source: _ } = info;
        println!("Mod Name: {name}");

        let mod_link = format!("https://api.modrinth.com/v2/project/{id}/version");

        let mut res = client.get(mod_link)
                            .query(&[
                                ("include_changelog", "false"),
                                ("loaders", &format!("[\"{loader}\"]")),
                                ("game_versions", &format!("[\"{version}\"]"))
                            ])
                            .send()?;

        let mut body = String::new();

        res.read_to_string(&mut body)?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!("Body:\n{}\n", body);
    }

    Ok(())
}

#[derive(Serialize)]
struct _ApiResponse<T> {
    success: bool,
    data: T,
    error: Option<String>,
}

pub fn _parse_modrinth() -> Result<()> {

    // need config id, version, and modloader
    // output -> Option<ModrinthVersion> ?
    // deserialize JSON array into Vec<ModrinthVersion>
    // chekc if it is empty,,,, extract top version to find the download url

    Ok(())
}
