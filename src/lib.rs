use anyhow::Result;
use std::io::ErrorKind;
use log::{info, warn, error};

pub mod cli;
pub mod config;
pub mod modrinth;

pub fn run() -> Result<()> {
    let client = modrinth::create_client()?;

    let config_file: String = match config::read() {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            warn!("No manifest found in cwd");
            info!("Writing default manifest into cwd");

            config::create_config();
            return Ok(());
        }

        Err(e) => {
            return Err(e.into()); // cast io error to anyhow error i think??
        }
    };

    let (mods, version, modloader) = match config::parse(config_file) {
        Ok(c) => c,
        Err(e) => {
            error!("Error: {e}");

            return Ok(());
        }
    };

    let responses = modrinth::call_modrinth(mods, version, modloader, client).expect("POOP");
    let links = modrinth::parse_modrinth(responses).expect("super poop");

    let mut download_prompt = String::new();

    println!("Would you like to download the files?(y/n)");
    std::io::stdin()
        .read_line(&mut download_prompt)
        .expect("Failed to read line");

    if download_prompt.trim() == "y" {
        modrinth::download_file(links);
    }
    Ok(())
}
