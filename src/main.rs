use minecraftthing::run;
use env_logger;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
