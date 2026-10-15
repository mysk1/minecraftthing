use anyhow::Result;
use std::io::ErrorKind;

mod config;
mod modrinth;

fn main() -> Result<()> {
    let client = modrinth::create_client();
    let config_file: String = match config::read() {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("No manifest found in cwd");
            println!("Writing default manifest into cwd");

            return Ok(());
        },

        Err(e) => {
            return Err(e.into()); // cast io error to anyhow error i think??
        }
    };

    let (mods, version, modloader) = match config::parse(config_file) {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {e}");

            return Ok(());
        }
    };

    let responses = modrinth::call_modrinth(mods, version, modloader, client).expect("POOP");
    let links = modrinth::parse_modrinth(responses).expect("super poop");

    let mut download_prompt = String::new();

    println!("Would you like to download the files?");
    std::io::stdin()
        .read_line(&mut download_prompt)
        .expect("Failed to read line");

    if download_prompt.trim() == "y" {
        modrinth::download_file(links);
    }

    Ok(())
}
