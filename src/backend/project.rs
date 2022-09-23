use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use crate::interface::theme::apply;
use crate::interface::theme::Theme;

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
    pub fn init(self, paths: Vec<PathBuf>, force_deep_sync: bool, config: &Configuration) -> Self {
        match std::fs::read_to_string(Path::new(
            &(BaseDirs::new()
                .unwrap()
                .home_dir()
                .to_str()
                .unwrap()
                .to_string()
                + PROJECT_STORE_LOCATION),
        )) {
            Ok(path) => match self.load(path) {
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
                    self.index(None, paths)
                } else {
                    self.index(Some(IndexLevel::Deep), paths)
                }
            }
        }
    }

    ///
    /// Save Self to $HOME/.config/hop/projects.json
    ///
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

    ///
    /// Load Self from $HOME/.config/hop/projects.json
    ///
    pub fn load(mut self, path: String) -> Result<Self, serde_json::Error> {
        match serde_json::from_str::<ProjectList>(&path) {
            Ok(project_list) => {
                self.projects = project_list.projects;
                return Ok(self);
            }
            Err(e) => return Err(e),
        }
    }

    ///
    /// Shallow compare Self with $HOME/.config/hop/projects.json
    ///
    fn shallow(&mut self, paths: Vec<PathBuf>) {
        for path in paths {
            if !self
                .projects
                .clone() // I want to remove this by using Lifetimes.
                .into_iter()
                .map(|p| p.path)
                .collect::<Vec<_>>()
                .contains(&path.to_str().unwrap().to_owned())
            {
                self.projects.push(apply(path.to_str().unwrap().to_owned()));
            }
        }
    }

    ///
    /// Index Self with an index level
    ///
    pub fn index(mut self, level: Option<IndexLevel>, paths: Vec<PathBuf>) -> Self {
        if let Some(level) = level {
            match level {
                IndexLevel::Shallow => {
                    self.shallow(paths);
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
