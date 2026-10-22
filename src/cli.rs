use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

//numma update
// -q quiet     quiet mode (accepts default choices automatically)
// -p pretend   pretend (shows proposed changes without applying)

//numma
// list           lists mod folders and info about what version theyre on
// select <path>  selects a mod folder for next commands(so you dont have to be in the directory?)
// search <thing> search for a certain mod(if a version isnt specified, list all)
// -v -vv         verbose

//numma set       specific to one mod folder(can all be set in config.toml)
// -d directory   set minecraft mods folder (searches for config.toml here too)
// -m mc-version  sets target minecraft version
// -r required    sets required mods(wont update any if this mod's version is missing)
// -t threshold   sets minimum percentage amount of mods available to be updated to update entire mod folder

//numma add     adds mods(for the future?)

//this might be total POO so u can change it if it is pookie

#[derive(Parser)]
#[command(version = "1.0", name = "numma", about = "A Minecraft mod updater")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// This is for an epic TEST!
    Test(TestArgs),

    /// Update your mods
    Update(UpdateArgs),
    /// List mod folders and current versions
    List(ListArgs),
    /// Select a default mod folder
    Select(SelectArgs),
    /// Sets a new mod folder
    Set(SetArgs),
    /// Add a new mod
    Add(AddArgs),
}
#[derive(Args)]
struct TestArgs {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    number: i32,
}

#[derive(Args)]
struct UpdateArgs {
    /// Quiet mode (accepts default choices without prompting)
    #[arg(short, long)]
    quiet: bool,

    /// Pretend mode (shows proposed changes without applying)
    #[arg(short, long)]
    pretend: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Args)]
struct ListArgs {
    //
}

#[derive(Args)]
struct SelectArgs {
    /// Path or identifier of mod folderr
    #[arg(help = "The path to the mod folder")]
    path: String,
}

#[derive(Args)]
struct SetArgs {
    /// Sets a new mod directory
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// Sets specific target minecraft version(defaults to newest)
    #[arg(default_value = "newest")]
    #[arg(short, long)]
    mc_version: Option<String>,

    /// Sets a required mods for update(by name or ID)
    #[arg(short, long)]
    required: Option<Vec<String>>,
}

#[derive(Args)]
struct AddArgs {
    // blegh!
}

pub fn cli_command() {
    let cli = Cli::parse();

    println!("yahoo");
}
