use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use crate::interface::theme::Theme;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub path: String,
    pub theme: Theme,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Store {
    pub projects: Vec<Project>,
}

pub fn read(path: &Path) -> Result<Store, serde_json::Error> {
    let projects = std::fs::read_to_string(&path).unwrap();

    serde_json::from_str::<Store>(&projects)
}

pub fn store(store: Store) -> Result<(), std::io::Error> {
    std::fs::write(
        Path::new("test.json"),
        serde_json::to_string_pretty(&store).unwrap(),
    )
}
