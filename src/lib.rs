use anyhow::Result;

pub mod cli;
pub mod config;
pub mod modrinth;

pub mod commands;

use cli::Commands;

pub fn run() -> Result<()> {
    let args = cli::cli_parse().unwrap_or_else(|error| error.exit());

    match args.command {
        Commands::Update(update_args) => commands::update::execute(update_args)?,
        Commands::List(list_args) => commands::list::execute(list_args)?,
        Commands::Select(_select) => {}
        Commands::Set(_set) => {}
        Commands::Add(_add) => {}
    }

    Ok(())
}
