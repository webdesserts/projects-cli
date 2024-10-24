use crate::config::{Config, ProjectRootSet, ProjectSet};
use crate::utils;
use failure::Error;
use std::fs;

pub fn list(paths: bool, config: Config) -> Result<(), Error> {
    let paths = if paths {
        list_roots(&config)?
    } else {
        list_projects(&config)?
    };
    let sorted_paths = utils::sort_set(paths);
    for path in sorted_paths {
        println!("{}", path.display());
    };
    Ok(())
}

pub fn list_roots(config: &Config) -> Result<ProjectRootSet, Error> {
    if config.paths.is_empty() {
        bail!("You haven't configured any paths yet! Use the \"add\" command to add some.")
    } else {
        Ok(config.paths.clone())
    }
}

pub fn list_projects(config: &Config) -> Result<ProjectSet, Error> {
    let mut projects = ProjectSet::new();

    for path in &config.paths {
        let dir = fs::read_dir(path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                format_err!("Project root not found, consider removing: \n{}", path.display())
            } else {
                error.into()
            }
        })?;
        
        for entry in dir {
            let path = entry?.path();
            if path.is_dir() {
                projects.insert(path);
            }
        }
    }

    Ok(projects)
}
