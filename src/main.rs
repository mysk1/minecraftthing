use anyhow::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    println!("Please input the name of a mod from modrinth");
    println!("e.g given https://modrinth.com/mod/sodium, input 'sodium'");

    let mut mod_name = String::new();

    io::stdin()
      .read_line(&mut mod_name)
      .expect("Failed to read line");

    let mod_link = format!("https://api.modrinth.com/v2/project/{mod_name}");

    let mut res = reqwest::blocking::get(mod_link)?;
    let mut body = String::new();

    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
