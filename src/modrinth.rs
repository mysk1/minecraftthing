use anyhow::Result;
use std::io::{Read};
use serde::{Serialize};

pub fn call_modrinth(id: String, version: &String, loader: &String) -> Result<()> {
    let mod_link = format!("https://api.modrinth.com/v2/project/{id}/version?loaders={loader}game_versions={version}&include_changelog=false");
    let mut res = reqwest::blocking::get(mod_link)?;
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
