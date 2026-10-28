// list commands
use crate::cli::ListArgs;
use crate::config;

pub fn execute(_args: ListArgs) -> anyhow::Result<()> {
    let config_parsed = config::load()?;

    config::list(&config_parsed);
    Ok(())
}
