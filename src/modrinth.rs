use anyhow::Result;
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;

use std::collections::HashMap;
use std::io::Read;

use crate::config::Mod;

pub fn create_client() -> Client {
    let client = Client::new();
    return client;
}

pub fn call_modrinth(
    mods: HashMap<String, Mod>,
    version: String,
    loader: String,
    client: Client,
) -> Result<Vec<String>> {
    let mut responses: Vec<String> = Vec::new();

    for (name, info) in mods {
        let Mod { id, source: _ } = info;
        println!("Mod Name: {name}");

        let mod_link = format!("https://api.modrinth.com/v2/project/{id}/version");

        let mut res = client
            .get(mod_link)
            .query(&[
                ("include_changelog", "false"),
                ("loaders", &format!("[\"{loader}\"]")),
                ("game_versions", &format!("[\"{version}\"]")),
            ])
            .send()?;

        let mut body = String::new();

        res.read_to_string(&mut body)?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!("Body:\n{}\n", body);

        responses.push(body);
    }

    Ok(responses)
}

#[derive(Serialize)]
struct _ApiResponse<T> {
    success: bool,
    data: T,
    error: Option<String>,
}

pub fn parse_modrinth(responses: Vec<String>) -> Result<Vec<(String, String)>> {
    let mut url_list: Vec<(String, String)> = Vec::new();
    for json in &responses {
        let parsed: Value = serde_json::from_str(json)?;

        let url = parsed[0]["files"][0]["url"].to_string();
        let filename = parsed[0]["files"][0]["filename"].to_string();

        url_list.push((url, filename));
    }

    Ok(url_list)
}

pub fn download_file(url_list: Vec<(String, String)>) -> Result<()> {
    for (url, filename) in url_list {
        let url_parsed = rem_first_and_last(&url);
        let filename_parsed = rem_first_and_last(&filename);

        let resp = reqwest::blocking::get(url_parsed).expect("request failed");
        let body = resp.text().expect("body invalid");
        let mut out = std::fs::File::create(filename_parsed).expect("failed to create file");
        std::io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    }

    Ok(())
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
