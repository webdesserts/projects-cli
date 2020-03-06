mod commands;
mod config;
mod utils;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;

use exitfailure::ExitFailure;
use std::path::PathBuf;
use structopt::StructOpt;

/// Manages a list of projects throughout your file system
#[derive(StructOpt)]
#[structopt(name = "projects-cli")]
struct App {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Track projects in the given directory
    #[structopt(name = "track")]
    Track {
        #[structopt(parse(from_os_str), default_value = ".")]
        path: PathBuf,
    },
    /// Stops tracking projects in the given directory
    #[structopt(name = "remove")]
    Remove {
        #[structopt(parse(from_os_str))]
        /// The path to stop tracking. If no path is passed, we'll display a list or track directories to select from.
        path: Option<PathBuf>,
    },
    /// Displays the current list of tracked directories
    #[structopt(name = "list")]
    List {
        #[structopt(long = "paths")]
        paths: bool,
    },
    /// Displays searchable menu of all projects. Will return the selected project's path
    #[structopt(name = "select")]
    Select,
    /// Prints a shell script that can be used to enable jumping to project directories
    #[structopt(name = "init")]
    Init {
        /// What shell are you initializing in? Right now only supports "bash"
        #[structopt(default_value = "bash")]
        shell: commands::Shells,
        /// Don't automatically define the "p" alias
        #[structopt(long = "no-alias")]
        no_alias: bool,
    },
}

fn main() -> Result<(), ExitFailure> {
    let config = config::load()?;
    let app = App::from_args();
    match app.cmd {
        Command::Track { path } => commands::track(path, config)?,
        Command::Remove { path } => commands::remove(path, config)?,
        Command::List { paths } => commands::list(paths, config)?,
        Command::Select => commands::select(config)?,
        Command::Init { shell, no_alias } => commands::init(shell, no_alias)?,
    };
    Ok(())
}
