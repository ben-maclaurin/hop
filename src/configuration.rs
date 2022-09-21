use config::Config;
use std::collections::HashMap;

use crate::directory_manager::get_home_dir;

#[derive(Clone)]
pub struct Configuration {
    pub directory: String,
    pub editor: String,
    pub title: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            directory: String::from(""),
            editor: String::from("vim"),
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
                "directory" => self.directory = value,
                "title" => self.title = value,
                _ => self.editor = value,
            }
        }
    }
}
