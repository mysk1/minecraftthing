use anyhow::Result;
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;

use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use crate::config::ModInfo;

pub fn create_client() -> Result<Client> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .user_agent("NUMMA")
        .build()?;
    Ok(client)
}

#[derive(Default)]
pub struct Mod {
    pub source: String,
    pub id: String,
    pub name: String,
    pub api_body: String,
    pub filename: String,
    pub url: String,
    pub required: bool,
    pub available: bool,
}

// given a list of mods, query all of them on modrinth
pub fn call_modrinth(
    mods: HashMap<String, ModInfo>,
    version: &str,
    loader: &str,
    client: &Client,
) -> Result<Vec<Mod>> {
    let mut responses: Vec<Mod> = Vec::new();

    for (name, info) in mods {
        let ModInfo {
            id,
            source,
            required,
        } = info;

        let mod_link = format!("https://api.modrinth.com/v2/project/{id}/version");

        let mut res = client
            .get(mod_link)
            .query(&[
                ("include_changelog", "false"),
                ("loaders", &format!("[\"{loader}\"]")),
                ("game_versions", &format!("[\"{version}\"]")),
            ]) // only include the stuff we want, keep it snappy
            .send()?;

        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let required_matched;
        if let Some(req) = required {
            required_matched = req;
        } else {
            required_matched = false;
        };

        let mod_summary = Mod {
            required: required_matched,
            source,
            id: id.clone(),
            name: name.clone(),
            api_body: body,
            ..Default::default()
        };

        responses.push(mod_summary);
    }

    Ok(responses)
}

#[derive(Serialize)]
struct _ApiResponse<T> {
    success: bool,
    data: T,
    error: Option<String>,
}

pub fn parse_modrinth(responses: Vec<Mod>) -> Result<Vec<Mod>> {
    let mut vec_mods: Vec<Mod> = Vec::new();

    for entry in responses {
        let parsed: Value = serde_json::from_str(&entry.api_body)?;

        // kinda bad but probably overkill to make a whole typed data structure
        let url = parsed[0]["files"][0]["url"].to_string();
        let filename = parsed[0]["files"][0]["filename"].to_string();

        let available = if url == "null" { false } else { true };

        let mod_summary = Mod {
            url,
            filename,
            available,
            ..entry
        };

        vec_mods.push(mod_summary);
    }

    Ok(vec_mods)
}

pub fn generate_summary(vec_mods: Vec<Mod>) -> Result<(Vec<(String, String)>, f64)> {
    let count_total = vec_mods.len();
    let mut count_found = 0u16;
    let mut downloadable_mods = Vec::new();

    for Mod {
        name,
        url,
        filename,
        available,
        required,
        ..
    } in vec_mods
    {
        if !available && required {
            anyhow::bail!(
                "The mod {name} was marked as required but wasn't found, cancelling operation."
            )
        };

        if available {
            println!("{name}: \x1b[32mFOUND ✔\x1b[0m");
            downloadable_mods.push((url, filename));
            count_found += 1;
        } else {
            println!("{name}: \x1b[31mNOT FOUND ✗\x1b[0m");
        };
    }

    println!("Found {count_found} out of {count_total} mods available to download");

    let ratio = count_found as f64 / count_total as f64;

    Ok((downloadable_mods, ratio))
}

pub fn download_files(links: Vec<(String, String)>) -> Result<()> {
    for link in links {
        let (url, filename) = link;

        // knock on from .to_string() above
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
