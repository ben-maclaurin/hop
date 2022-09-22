use std::path::Path;

use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use crate::interface::theme::Theme;

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
