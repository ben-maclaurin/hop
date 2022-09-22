use config::Config;
use config::Environment;
use config::File;
use directories::BaseDirs;
use std::collections::HashMap;

const CONFIG_LOCATION: &'static str = "/.config/hop/hop.yml";

#[derive(Clone)]
pub struct Configuration {
    pub directory: String,
    pub editor: String,
    pub title: String,
    pub icons: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            directory: String::from("jump"),
            editor: String::from("code"),
            title: String::from("Hop"),
            icons: false,
        }
    }
}

impl Configuration {
    pub fn init(&mut self) {
        let config = Config::builder()
            .add_source(File::with_name(
                &(BaseDirs::new()
                    .unwrap()
                    .home_dir()
                    .to_str()
                    .expect("could not find home dir")
                    .to_string()
                    + CONFIG_LOCATION),
            ))
            .add_source(Environment::with_prefix("APP"))
            .build();

        match config {
            Ok(config) => {
                self.set_values(deserialize_config(config));
            }
            Err(_) => {} // use defaults
        }
    }

    fn set_values(&mut self, config: HashMap<String, String>) {
        for (name, value) in config {
            match name.as_str() {
                "directory" => self.directory = value,
                "title" => self.title = value,
                "icons" => {
                    self.icons = match value.as_str() {
                        "true" => true,
                        _ => false,
                    }
                }
                _ => self.editor = value,
            }
        }
    }
}

fn deserialize_config(config: Config) -> HashMap<String, String> {
    config
        .try_deserialize::<HashMap<String, String>>()
        .expect("error reading config")
}
