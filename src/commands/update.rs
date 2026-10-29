use log::{error, info, warn};

use crate::cli::UpdateArgs;
use crate::config;
use crate::modrinth;

pub fn execute(_args: UpdateArgs) -> anyhow::Result<()> {
    let client = modrinth::create_client()?;

    let config = config::load()?;

    let mut responses = Vec::new();
    for i in 1..=5 {
        match modrinth::call_modrinth(
            config.mods.clone(),
            &config.version,
            &config.modloader,
            &client,
        ) {
            Ok(c) => {
                responses = c;
                break;
            }
            Err(e) => {
                warn!("Failed to fetch from modrinth with: {e}");
                info!("Trying again, try {i}/5");

                if i == 5 {
                    return Err(e);
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

    let (links, ratio) = match modrinth::generate_summary(vec_mods) {
        Ok(c) => c,
        Err(e) => {
            error!("Error: {e}");

            return Ok(());
        }
    };

    let download_prompt;

    match config.threshold {
        Some(v) => {
            download_prompt = ratio > v;
        }
        None => {
            download_prompt = modrinth::prompt_download()?;
        }
    }

    if download_prompt {
        println!("Downloading mods...");
        modrinth::download_files(links)?;
    }
    Ok(())
}
