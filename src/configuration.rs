use config::Config;
use std::collections::HashMap;

use crate::directory_manager::get_home_dir;

#[derive(Clone)]
pub struct Configuration {
    pub projects_dir: String,
    pub launch_command: String,
    pub title: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            projects_dir: String::from(""),
            launch_command: String::from("vim"),
            title: String::from("Project directories"),
        }
    }
}

impl Configuration {
    pub fn init(&mut self) {
        let config = Config::builder()
            .add_source(config::File::with_name(
                &(String::from(get_home_dir().to_str().unwrap()) + "/.config/jump/jump.yml"),
            ))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let config = config.try_deserialize::<HashMap<String, String>>().unwrap();

        for (name, value) in config {
            match name.as_str() {
                "projects_dir" => self.projects_dir = value,
                "title" => self.title = value,
                _ => self.launch_command = value,
            }
        }
    }
}
