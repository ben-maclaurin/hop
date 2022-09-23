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

pub trait Empty {
    fn empty() -> Self;
}

impl Empty for Project {
    fn empty() -> Self {
        Self {
            path: "".to_string(),
            theme: Theme::empty(),
            language: "".to_string(),
        }
    }
}

impl Empty for ProjectList {
    fn empty() -> Self {
        Self {
            projects: vec![Project::empty()],
        }
    }
}

pub enum IndexLevel {
    Shallow,
    Deep,
}

impl ProjectList {
    pub fn save(&self) -> Result<(), std::io::Error> {
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

    pub fn load(mut self, path: String) -> Result<Self, serde_json::Error> {
        match serde_json::from_str::<ProjectList>(&path) {
            Ok(project_list) => {
                self = project_list;
                return Ok(self);
            }
            Err(e) => return Err(e),
        }
    }

    pub fn index(mut self, level: Option<IndexLevel>, paths: Vec<PathBuf>) -> Self {
        if let Some(level) = level {
            match level {
                IndexLevel::Shallow => {
                    for path in paths {
                        if !self
                            .projects
                            .clone()
                            .into_iter()
                            .map(|p| p.path)
                            .collect::<Vec<_>>()
                            .contains(&path.to_str().unwrap().to_owned())
                        {
                            self.projects.push(apply(path.to_str().unwrap().to_owned()));
                        }
                    }

                    self.save().unwrap();
                }
                IndexLevel::Deep => {
                    for path in paths {
                        self.projects.push(apply(path.to_str().unwrap().to_owned()));
                    }

                    self.save().unwrap();
                }
            }
        } else {
            for path in paths {
                self.projects.push(Project {
                    path: path.to_str().unwrap().to_string(),
                    theme: Theme::empty(),
                    language: "".to_string(),
                });
            }
        }

        self
    }
}

pub fn get_project_list(
    paths: Vec<PathBuf>,
    force_deep_sync: bool,
    config: &Configuration,
) -> ProjectList {
    let project_list = ProjectList::empty();

    match std::fs::read_to_string(Path::new(
        &(BaseDirs::new()
            .unwrap()
            .home_dir()
            .to_str()
            .unwrap()
            .to_string()
            + PROJECT_STORE_LOCATION),
    )) {
        Ok(path) => match project_list.load(path) {
            Ok(list) => {
                if !config.icons {
                    return list.index(None, paths);
                } else if force_deep_sync {
                    return list.index(Some(IndexLevel::Deep), paths);
                } else {
                    return list.index(Some(IndexLevel::Shallow), paths);
                }
            }
            Err(_) => return ProjectList::empty(),
        },
        Err(_) => {
            if !config.icons {
                project_list.index(None, paths)
            } else {
                project_list.index(Some(IndexLevel::Deep), paths)
            }
        }
    }
}

pub fn get_project_paths(config: &Configuration) -> Result<Vec<PathBuf>, std::io::Error> {
    let home_dir = BaseDirs::new()
        .unwrap()
        .home_dir()
        .to_str()
        .unwrap()
        .to_string()
        + "/"
        + &config.directory;

    let target_dir = Path::new(&home_dir);

    let projects = fs::read_dir(target_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    projects
}
