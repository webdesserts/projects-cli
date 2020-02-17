use std::io::Cursor;
use std::path::{PathBuf, Path};
use failure::{Error};
use skim::{SkimOptionsBuilder, Skim};
use crate::config::{Config, ProjectSet};
use crate::commands::list::{list_projects};

pub fn select(config: Config) -> Result<(), Error> {
    if config.paths.is_empty() {
        bail!("You haven't configured any paths yet! Use the \"add\" command to add some.");
    }
    let projects = get_project_names(list_projects(config)?);
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

fn get_project_names (projects: ProjectSet) -> Vec<ProjectDetails> {
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