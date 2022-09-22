use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use crate::interface::theme::Theme;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::backend::configuration::Configuration;

pub const PROJECT_STORE_LOCATION: &'static str = "/.config/hop/projects.json";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub path: String,
    pub theme: Theme,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Store {
    pub projects: Vec<Project>,
}

pub fn read(path: &Path) -> Option<Store> {
    match std::fs::read_to_string(&path) {
        Ok(string) => Some(serde_json::from_str::<Store>(&string).unwrap()),
        Err(_) => None,
    }
}

pub fn store(store: Store) -> Result<(), std::io::Error> {
    std::fs::write(
        Path::new(
            &(BaseDirs::new()
                .unwrap()
                .home_dir()
                .to_str()
                .unwrap()
                .to_string()
                + PROJECT_STORE_LOCATION),
        ),
        serde_json::to_string_pretty(&store).unwrap(),
    )
}

pub fn get_projects(config: &Configuration) -> Result<Vec<PathBuf>, std::io::Error> {
    let home_dir = BaseDirs::new()
        .unwrap()
        .home_dir()
        .to_str()
        .unwrap()
        .to_string()
        + "/"
        + &config.directory; // also bad

    let target_dir = Path::new(&home_dir);

    let projects = fs::read_dir(target_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    projects
}

