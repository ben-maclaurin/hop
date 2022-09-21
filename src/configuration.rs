use config::Config;
use std::collections::HashMap;

use crate::directory_manager::get_home_dir;

const CONFIG_LOCATION: &'static str = "/.config/hop/hop.yml";

#[derive(Clone)]
pub struct Configuration {
    pub directory: String,
    pub editor: Editor,
    pub title: String,
}

#[derive(Clone)]
pub enum Editor {
    Vim,
    Neovim,
    VSCode,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            directory: String::from("jump"),
            editor: Editor::VSCode,
            title: String::from("Hop"),
        }
    }
}

impl Configuration {
    pub fn init(&mut self) {

        let config: Option<Config> = match Config::builder()
            .add_source(config::File::with_name(
                &(String::from(get_home_dir().to_str().unwrap()) + CONFIG_LOCATION),
            ))
            .add_source(config::Environment::with_prefix("APP"))
            .build() {
                Ok(config) => Some(config),
                Err(_) => None,
            };

        match config {
            Some(config) => {
                let config = config.try_deserialize::<HashMap<String, String>>().unwrap();

                for (name, value) in config {
                    match name.as_str() {
                        "directory" => self.directory = value,
                        "title" => self.title = value,
                        _ => self.editor = get_editor(value),
                    }
                }
            },
            _ => {},
        }

    }
}

pub fn get_launch_cmd(editor: Editor) -> String {
    match editor {
        Editor::Vim => "vim".to_string(),
        Editor::VSCode => "code".to_string(),
        _ => "nvim".to_string()
    }
}

fn get_editor(configuration_value: String) -> Editor {
    match configuration_value.as_str() {
        "vim" => return Editor::Vim,
        "neovim" => return Editor::Neovim,
        _ => Editor::VSCode,
    }
}
