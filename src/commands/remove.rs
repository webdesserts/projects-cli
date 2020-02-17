use std::path::{PathBuf};
use crate::config::{Config};
use crate::utils;

pub fn remove(path: PathBuf, mut config: Config) -> Result<(), failure::Error> {
    let full_path = utils::normalize_path(&std::env::current_dir()?.join(path));
    let removed = config.paths.remove(&full_path);
    if removed {
        println!("No longer tracking {:?}", full_path);
        config.store()?;
    } else {
        println!("Not currently tracking {:?}", full_path);
    }
    Ok(())
}