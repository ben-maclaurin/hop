use directories::BaseDirs;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::configuration::Configuration;

pub fn get_entries(config: Configuration) -> Result<Vec<PathBuf>, std::io::Error> {
    let home_dir = BaseDirs::new()
        .unwrap()
        .home_dir()
        .to_str()
        .unwrap()
        .to_string()
        + "/"
        + &config.directory; // also bad

    let target_dir = Path::new(&home_dir);

    let entries = fs::read_dir(target_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    entries
}
