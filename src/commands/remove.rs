use std::path::{PathBuf};
use dialoguer;
use crate::config::{Config};
use crate::utils;
use crate::commands::list::{list_roots};

pub fn remove(path: Option<PathBuf>, mut config: Config) -> Result<(), failure::Error> {
    if path == None && config.paths.is_empty() {
        println!("You are not currently tracking any directories. You can track projects in a");
        println!("directory by using the `projects track` command:");
        println!("");
        println!("  projects track ~/code/");
        println!("");
        return Ok(())
    }

    let p = match path {
        Some(path) => path,
        None => select_root(&config)?
    };

    let full_path = utils::normalize_path(&std::env::current_dir()?.join(p));
    let removed = config.paths.remove(&full_path);
    if removed {
        println!("No longer tracking {:?}", full_path);
        config.store()?;
    } else {
        println!("Not currently tracking {:?}", full_path);
    }
    Ok(())
}

/**
 * @todo what if there are no projects?
 */
pub fn select_root(config: &Config) -> Result<PathBuf, failure::Error> {
    println!("You are currently tracking projects in the following directories. Which would you like to remove?");
    let rootset = list_roots(config)?;
    let mut roots: Vec<&PathBuf> = rootset.iter().collect();
    roots.sort();
    let path_strings: Vec<&str> = roots.iter().map(|root| root.to_str().unwrap()).collect();
    let mut selector = dialoguer::Select::new();
    selector.items(&path_strings);
    selector.default(0);
    let result = selector.interact()?;
    return Ok(roots[result].clone())
}