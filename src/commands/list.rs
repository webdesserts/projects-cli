use crate::config::{Config, ProjectRootSet};
use std::fs;
use failure::Error;

pub fn list(paths: bool, config: Config) -> Result<(), Error>{
    let paths = if paths {
        list_roots(&config)?
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

pub fn list_roots(config: &Config) -> Result<ProjectRootSet, Error>{
    if config.paths.is_empty() {
        bail!("You haven't configured any paths yet! Use the \"add\" command to add some.")
    } else {
        Ok(config.paths.clone())
    }
}

pub fn list_projects(config: &Config) -> Result<ProjectRootSet, Error> {
    let mut projects = ProjectRootSet::new();
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
