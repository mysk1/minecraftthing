use anyhow::Result;
use minecraftthing::run;

fn main() -> Result<()> {
    env_logger::init();

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
