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
pub struct ProjectList {
    pub projects: Vec<Project>,
}

pub fn read(path: &Path) -> Option<ProjectList> {
    match std::fs::read_to_string(&path) {
        Ok(string) => Some(serde_json::from_str::<ProjectList>(&string).unwrap()),
        Err(_) => None,
    }
}

trait New {
    fn new() -> Self;
}

impl New for Project {
    fn new() -> Self {
        Self {
            path: "".to_string(),
            theme: Theme {
                icon: " ".to_string(),
                color: WHITE,
            },
            language: "".to_string(),
        }
    }
}

impl New for ProjectList {
    fn new() -> Self {
        Self {
            projects: vec!(Project::new())
        }
    }
}

impl ProjectList {
    pub fn store(&self) -> Result<(), std::io::Error> {
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
            serde_json::to_string_pretty(&self).unwrap(),
        )
    }
}

pub fn get_project_list(
    entries: Vec<PathBuf>,
    force_deep_sync: bool,
    config: &Configuration,
) -> ProjectList {
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

pub fn no_sync(entries: Vec<PathBuf>) -> ProjectList {
    let mut store = ProjectList::new();

    for entry in entries {
        store.projects.push(Project {
            path: entry.to_str().unwrap().to_owned(),
            theme: Theme {
                icon: " ".to_string(),
                color: WHITE,
            },
            language: "".to_string(),
        })
    }

    store
}

pub fn shallow_sync(entries: Vec<PathBuf>) -> ProjectList {
    let mut current_list: ProjectList = read(Path::new(
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
        if !current_list
            .projects
            .clone()
            .into_iter()
            .map(|p| p.path)
            .collect::<Vec<_>>()
            .contains(&entry.to_str().unwrap().to_owned())
        {
            current_list
                .projects
                .push(apply(entry.to_str().unwrap().to_owned()));
        }
    }

    current_list.store().unwrap();

    current_list
}

pub fn deep_sync(entries: Vec<PathBuf>) -> ProjectList {
    let mut list = ProjectList::new(); 

    for entry in entries {
        list.projects.push(apply(entry.to_str().unwrap().to_owned()));
    }

    list.store().unwrap();

    list
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
