use failure::Error;
use std::str::FromStr;
use std::{env, io};

/*
 * Most of this code was blatantly stolen from the wonderful starship.rs project.
 */

const BASH_INIT: &str = include_str!("projects.bash");

pub fn init(shell: Shells) -> Result<(), Error> {
    let cli_path = path_to_projects_cli()?;
    match shell {
        Shells::Bash => println!("{}", BASH_INIT.replace("PROJECT_CLI_PATH", &cli_path)),
    }
    Ok(())
}

pub enum Shells {
    Bash,
}

impl FromStr for Shells {
    type Err = failure::Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "bash" => Ok(Shells::Bash),
            _ => bail!("Invalid shell \"{}\"", string),
        }
    }
}

fn path_to_projects_cli() -> io::Result<String> {
    let current_exe = env::current_exe()?
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "can't convert to str"))?
        .to_string()
        .replace("\"", "\"'\"'\"");
    Ok(current_exe)
}
