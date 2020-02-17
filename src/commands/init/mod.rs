use failure::Error;
use std::str::FromStr;

pub fn init(shell: Shells) -> Result<(), Error> {
    match shell {
        Shells::Bash => println!("{}", BASH_INIT)
    }
    Ok(())
}

pub enum Shells {
    Bash
}

impl FromStr for Shells {
    type Err = failure::Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "bash" => Ok(Shells::Bash),
            _ => bail!("Invalid shell \"{}\"", string)
        }
    }
}

const BASH_INIT: &str = include_str!("projects.bash");