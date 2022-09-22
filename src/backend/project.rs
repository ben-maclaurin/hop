use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use crate::interface::theme::apply;
use crate::interface::theme::Theme;
use crate::interface::theme::WHITE;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::backend::configuration::Configuration;

pub const PROJECT_STORE_LOCATION: &'static str = "/.config/hop/projects.json";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Project {
    pub path: String,
    pub theme: Theme,
    pub language: String,
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

pub fn get_projects(
    entries: Vec<PathBuf>,
    force_deep_sync: bool,
    config: &Configuration,
) -> Vec<Project> {
    match read(Path::new(
        &(BaseDirs::new()
            .unwrap()
            .home_dir()
            .to_str()
            .unwrap()
            .to_string()
            + PROJECT_STORE_LOCATION),
    )) {
        Some(_) => {
            if !config.icons {
                return no_sync(entries);
            } else if force_deep_sync {
                return deep_sync(entries);
            } else {
                return shallow_sync(entries);
            }
        }
        None => {
            if !config.icons {
                return no_sync(entries);
            } else {
                return deep_sync(entries);
            }
        }
    };
}

pub fn no_sync(entries: Vec<PathBuf>) -> Vec<Project> {
    let mut projects = Vec::<Project>::new();

    for entry in entries {
        projects.push(Project {
            path: entry.to_str().unwrap().to_owned(),
            theme: Theme {
                icon: " ".to_string(),
                color: WHITE,
            },
            language: "".to_string(),
        })
    }

    projects
}

// I don't like the amount of "cloning" in this function.
// Can it be improved?
pub fn shallow_sync(entries: Vec<PathBuf>) -> Vec<Project> {
    let mut current_store: Store = read(Path::new(
        &(BaseDirs::new()
            .unwrap()
            .home_dir()
            .to_str()
            .unwrap()
            .to_string()
            + PROJECT_STORE_LOCATION),
    ))
    .unwrap();

    for entry in entries {
        if !current_store
            .projects
            .clone()
            .into_iter()
            .map(|p| p.path)
            .collect::<Vec<_>>()
            .contains(&entry.to_str().unwrap().to_owned())
        {
            current_store
                .projects
                .push(apply(entry.to_str().unwrap().to_owned()));
        }
    }

    store(Store {
        projects: current_store.projects.clone(),
    })
    .unwrap();

    current_store.projects
}

pub fn deep_sync(entries: Vec<PathBuf>) -> Vec<Project> {
    let mut projects = Vec::<Project>::new();

    for entry in entries {
        projects.push(apply(entry.to_str().unwrap().to_owned()));
    }

    store(Store {
        projects: projects.clone(),
    })
    .unwrap();

    projects
}

pub fn get_project_paths(config: &Configuration) -> Result<Vec<PathBuf>, std::io::Error> {
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
