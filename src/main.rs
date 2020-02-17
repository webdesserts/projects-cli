mod utils;
mod commands;
mod config;

#[macro_use] extern crate serde;
#[macro_use] extern crate failure;

use std::path::{PathBuf};
use structopt::StructOpt;
use exitfailure::ExitFailure;

/// Manages a list of projects throughout your file system
#[derive(StructOpt)]
#[structopt(name = "projects-cli")]
struct App {
    #[structopt(subcommand)]
    cmd: Option<Command>
}

#[derive(StructOpt)]
enum Command {
    /// Track projects in the given directory, and add them to the search
    #[structopt(name = "track")]
    Track {
        #[structopt(parse(from_os_str), default_value = ".")]
        path: PathBuf,
    },
    /// Stop tracking projects in the given directory
    #[structopt(name = "remove")]
    Remove {
        #[structopt(parse(from_os_str), default_value = ".")]
        path: PathBuf,
    },
    /// Displays the list of saved paths expedite watches
    #[structopt(name = "list")]
    List {
        #[structopt(long = "paths")]
        paths: bool
    },
    #[structopt(name = "init",)]
    Init {
        #[structopt(default_value = "bash")]
        shell: commands::Shells
    }
}

fn main() -> Result<(), ExitFailure> {
    let config = config::load()?;
    let app = App::from_args();
    match app.cmd  {
        Some(Command::Track{path}) => commands::track(path, config)?,
        Some(Command::Remove{path}) => commands::remove(path, config)?,
        Some(Command::List{paths}) => commands::list(paths, config)?,
        Some(Command::Init{shell}) => commands::init(shell)?,
        None => commands::select(config)?
    };
    Ok(())
}
