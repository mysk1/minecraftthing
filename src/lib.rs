use anyhow::Result;

pub mod cli;
pub mod config;
pub mod modrinth;

pub mod commands;

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

        Commands::Update(update_args) => commands::update::execute(update_args)?,

        Commands::List(_list) => {
            let config_parsed = config::load()?;

            config::list(&config_parsed);
        }

        Commands::Select(_select) => {}
        Commands::Set(_set) => {}
        Commands::Add(_add) => {}
    }

    Ok(())
}
