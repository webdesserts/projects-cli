use std::collections::{HashSet};
use std::path::{PathBuf};
use webdesserts_confy as confy;

static APP_NAME: &str = "projects";

pub type ProjectSet = HashSet<PathBuf>;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: u8,
    pub paths: ProjectSet
}

impl Config {
    pub fn store(&self) -> Result<(), confy::ConfyError>{
        confy::store(APP_NAME, self)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            paths: ProjectSet::new()
        }
    }
}

pub fn load() -> Result<Config, confy::ConfyError> {
    confy::load(APP_NAME)
}
