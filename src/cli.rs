use clap::{Args, Parser, Subcommand};
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

#[derive(Debug, Parser)]
#[command(name = "numma")]
#[command(version = "1.0")]
#[command(about = "A Minecraft mod updater", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

#[derive(Debug, Args)]
pub struct TestArgs {
    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub number: i32,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Quiet mode (accepts default choices without prompting)
    #[arg(short, long)]
    pub quiet: bool,

    /// Pretend mode (shows proposed changes without applying)
    #[arg(short, long)]
    pub pretend: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Lists all mod instances and mods inside
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct SelectArgs {
    /// Path or identifier of mod folderr
    #[arg(help = "The path to the mod folder")]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct SetArgs {
    /// Sets a new mod directory
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    /// Sets specific target minecraft version(defaults to newest)
    #[arg(default_value = "newest")]
    #[arg(short, long)]
    pub mc_version: Option<String>,

    /// Sets a required mods for update(by name or ID)
    #[arg(short, long)]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    // TODO: add this later
}

pub fn cli_parse() -> Result<Cli, clap::Error> {
    let args = Cli::try_parse();
    args
}
