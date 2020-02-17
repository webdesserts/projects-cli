mod utils;

#[macro_use] extern crate serde;
#[macro_use] extern crate failure;

use structopt::StructOpt;
use std::fs;
use std::default::{Default};
use std::path::{PathBuf, Path};
use std::collections::{HashSet};
use failure::Error;
use exitfailure::ExitFailure;

static APP_NAME: &str = "projects";

/// Manages a list of projects throughout your file system
#[derive(StructOpt)]
#[structopt(name = "p")]
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
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    version: u8,
    paths: HashSet<PathBuf>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            paths: HashSet::new()
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    let config: Config = webdesserts_confy::load(APP_NAME)?;
    let app = App::from_args();
    match app.cmd  {
        Some(Command::Track{path}) => track_path(path, config)?,
        Some(Command::Remove{path}) => remove_path(path, config)?,
        Some(Command::List{paths}) => list(paths, config)?,
        None => select_project(config)?
    };
    Ok(())
}

fn track_path(path: PathBuf, mut config: Config) -> Result<(), failure::Error> {
    let full_path = utils::normalize_path(&std::env::current_dir()?.join(path));

    // Test to see if the path is a readable dir
    fs::read_dir(&full_path)?;

    let inserted = config.paths.insert(full_path.clone());
    if inserted {
        println!("Now tracking {:?}", full_path);
    } else {
        println!("Already tracking {:?}", full_path);
    }
    webdesserts_confy::store(APP_NAME, config)?;
    Ok(())
}

fn remove_path(path: PathBuf, mut config: Config) -> Result<(), failure::Error> {
    let full_path = utils::normalize_path(&std::env::current_dir()?.join(path));
    let removed = config.paths.remove(&full_path);
    if removed {
        println!("No longer tracking {:?}", full_path);
        webdesserts_confy::store(APP_NAME, config)?;
    } else {
        println!("Not currently tracking {:?}", full_path);
    }
    Ok(())
}

fn list(paths: bool, config: Config) -> Result<(), Error>{
    let paths = if paths {
        list_paths(config)?
    } else {
        list_projects(&config)?
    };
    for path in paths {
        if let Some(string) = path.to_str() {
            println!("{}", string)
        }
    };
    Ok(())
}

fn list_paths(config: Config) -> Result<HashSet<PathBuf>, Error>{
    if config.paths.is_empty() {
        bail!("You haven't configured any paths yet! Use the \"add\" command to add some.")
    } else {
        Ok(config.paths)
    }
}

fn list_projects(config: &Config) -> Result<HashSet<PathBuf>, Error> {
    let mut projects = HashSet::new();
    for path in &config.paths {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                projects.insert(path);
            }
        }
    };
    Ok(projects)
}

use skim::{SkimOptionsBuilder, Skim};
use std::io::Cursor;

fn select_project(config: Config) -> Result<(), Error> {
    if config.paths.is_empty() {
        bail!("You haven't configured any paths yet! Use the \"add\" command to add some.");
    }
    let projects = get_project_names(list_projects(&config)?);
    if projects.is_empty() {
        bail!("Your configured paths are currently empty. Try adding some projects to them");
    }
    let names: Vec<String> = projects.iter().map(|project| format!("{} – {}", project.name, project.path.to_str().unwrap())).collect();
    let input = names.join("\n");
    let options = SkimOptionsBuilder::default()
        .nth(Some("1"))
        .delimiter(Some(" – "))
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        if let Some(path) = projects[item.get_index()].path.to_str() {
            println!("{}", path);
        }
    }
    Ok(())
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ProjectDetails {
    path: PathBuf,
    name: String
}

fn get_project_names (projects: HashSet<PathBuf>) -> Vec<ProjectDetails> {
    let mut project_names = Vec::new();
    for path in projects {
        let file_name = get_file_name(&path);
        if let Some(name) = file_name {
            project_names.push(ProjectDetails { path, name });
        }
    };
    project_names.sort();
    return project_names
}

fn get_file_name<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref().file_name()
        .and_then(|os_str| os_str.to_str())
        .map(|str_ref| str_ref.to_string())
}