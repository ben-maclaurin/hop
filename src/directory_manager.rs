use directories::BaseDirs;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

use crate::configuration::Configuration;

pub fn get_home_dir() -> PathBuf {
    BaseDirs::new().unwrap().home_dir().to_owned()
}

pub fn get_entries(config: Configuration) -> Result<Vec<PathBuf>, std::io::Error> {
    let home_dir = get_home_dir().to_str().unwrap().to_owned() + "/" + &config.projects_dir;

    let target_dir = Path::new(&home_dir);

    let entries = fs::read_dir(target_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    entries
}
