use anyhow::Result;

mod config;
mod modrinth;

fn main() -> Result<()> {
    //println!("Please input the name of a mod from modrinth");
    //println!("e.g given https://modrinth.com/mod/sodium, input 'sodium'");

    //let mut mod_name = String::new();

    //io::stdin()
    //    .read_line(&mut mod_name)
    //    .expect("Failed to read line");

    //modrinth::call_modrinth(mod_name).expect("Poo");

    let client = modrinth::create_client();
    let (mods, version, modloader) = config::parse();
    modrinth::call_modrinth(mods, version, modloader, client).expect("POOP");

    Ok(())
}
