use anyhow::Result;
use std::io::{Read};

pub fn call_modrinth(mod_name: String) -> Result<()> {
    let mod_link = format!("https://api.modrinth.com/v2/project/{mod_name}");

    let mut res = reqwest::blocking::get(mod_link)?;
    let mut body = String::new();

    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}\n", body);

    Ok(())
}
