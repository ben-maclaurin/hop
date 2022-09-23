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
    pub icons: bool,
    pub include_files: bool,
    pub show_search: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            directory: String::from("Developer"),
            editor: String::from("code"),
            icons: false,
            include_files: false,
            show_search: true,
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
                "icons" => self.icons = resolve_bool(value),
                "include_files" => self.include_files = resolve_bool(value),
                "show_search" => self.show_search = resolve_bool(value),
                _ => self.editor = value,
            }
        }
    }
}

fn resolve_bool(value: String) -> bool {
    match value.as_str() {
        "true" => true,
        _ => false,
    }
}

fn deserialize_config(config: Config) -> HashMap<String, String> {
    config
        .try_deserialize::<HashMap<String, String>>()
        .expect("error reading config")
}
