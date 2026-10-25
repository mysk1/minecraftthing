use anyhow::Result;
use log::{error, info, warn};
use std::io::ErrorKind;

pub mod cli;
pub mod config;
pub mod modrinth;

use cli::Commands;

pub fn run() -> Result<()> {
    let args = cli::cli_parse().unwrap_or_else(|error| error.exit());

    match args.command {
        Commands::Test(test) => {
            let mut greeting = String::from("HI!");
            match test.name {
                Some(name) => {
                    greeting = format!("Hello {name}");
                }
                None => {}
            }
            for _i in 1..test.number {
                println!("{greeting}");
            }
        }

        Commands::Update(_update) => {
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

            let config = match config::parse(config_file) {
                Ok(c) => c,
                Err(e) => {
                    error!("Error: {e}");

                    return Ok(());
                }
            };

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

            let (links, ratio) = match modrinth::generate_summary(vec_mods) {
                Ok(c) => c,
                Err(e) => {
                    error!("Error: {e}");

                    return Ok(());
                }
            };

            let mut download_prompt = String::new();

            match config.threshold {
                Some(v) => {
                    if ratio > v {
                        download_prompt = String::from("y");
                    } else {
                        download_prompt = String::from("n");
                    };
                }
                None => {
                    println!("Would you like to download the files?(y/n)");
                    std::io::stdin()
                        .read_line(&mut download_prompt)
                        .expect("Failed to read line");
                }
            }

            if download_prompt.trim() == "y" {
                modrinth::download_files(links);
            }
        }

        Commands::List(_list) => {
            let config_file: String = match config::read() {
                Ok(c) => c,
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    warn!("No manifest found in cwd");
                    info!(
                        "Writing default manifest into cwd, you should edit that file and run this again!"
                    );

                    println!("No manifest found in cwd, creating one");

                    config::create_config();
                    return Ok(());
                }

                Err(e) => {
                    return Err(e.into()); // cast io error to anyhow error i think??
                }
            };

            let config_parsed = match config::parse(config_file) {
                Ok(c) => c,
                Err(e) => {
                    error!("Error: {e}");

                    return Ok(());
                }
            };

            config::list(config_parsed);
        }
        Commands::Select(_select) => {}
        Commands::Set(_set) => {}
        Commands::Add(_add) => {}
    }

    Ok(())
}
