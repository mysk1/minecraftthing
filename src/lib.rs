use anyhow::Result;
use log::{error, info, warn};
use std::io::ErrorKind;

pub mod cli;
pub mod config;
pub mod modrinth;

pub fn run() -> Result<()> {

    let client = modrinth::create_client()?;

    let config_file: String = match config::read() {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            warn!("No manifest found in cwd");
            info!(
                "Writing default manifest into cwd, you should edit that file and run this again!"
            );

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

    let mut responses = Vec::new();
    for i in 1..=5 {
        match modrinth::call_modrinth(mods.clone(), &version, &modloader, &client) {
            Ok(c) => {
                responses = c;
                break;
            }
            Err(e) => {
                warn!("Failed to fetch from modrinth with: {e}");
                info!("Trying again, try {i}/5");

                if i == 5 {
                    return Err(e.into());
                } else {
                    continue;
                }
            }
        }
    }

    let vec_mods = match modrinth::parse_modrinth(responses) {
        Ok(c) => c,
        Err(e) => {
            error!("Error: {e}");

            return Ok(());
        }
    };

    let links = match modrinth::generate_summary(vec_mods) {
        Ok(c) => c,
        Err(e) => {
            error!("Error: {e}");

            return Ok(());
        }
    };

    let mut download_prompt = String::new();

    println!("Would you like to download the files?(y/n)");
    std::io::stdin()
        .read_line(&mut download_prompt)
        .expect("Failed to read line");

    if download_prompt.trim() == "y" {
        modrinth::download_files(links);
    }
    Ok(())
}
