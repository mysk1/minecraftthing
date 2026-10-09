use anyhow::Result;
use std::io::{Read};
use serde::{Serialize};

use std::collections::HashMap;


pub fn call_modrinth(id: String, version: &String, loader: &String) -> Result<()> {
    let mut params: HashMap<String, bool> = HashMap::new();

    params.insert(String::from("include_changelog"), true);

    let mod_link = format!("https://api.modrinth.com/v2/project/{id}/version");

    let client = reqwest::blocking::Client::new();

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
