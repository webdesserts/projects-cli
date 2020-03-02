use crate::config::Config;
use crate::utils;
use std::fs;
use std::path::PathBuf;

pub fn track(path: PathBuf, mut config: Config) -> Result<(), failure::Error> {
    let full_path = utils::normalize_path(&std::env::current_dir()?.join(path));

    // Test to see if the path is a readable dir
    fs::read_dir(&full_path)?;

    let inserted = config.paths.insert(full_path.clone());
    if inserted {
        println!("Now tracking {:?}", full_path);
    } else {
        println!("Already tracking {:?}", full_path);
    }
    config.store()?;
    Ok(())
}
