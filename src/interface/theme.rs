use std::path::Path;

use crate::backend::{
    linguist::{get_git_language_of_path, Language},
    project::Project,
};
use serde_derive::{Deserialize, Serialize};

pub mod kanagawa;
use kanagawa::Kanagawa;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Theme {
    pub icon: String,
    pub color: (u8, u8, u8)
}

pub trait Definition {
    fn init(lanauge: Language) -> Theme;
}

pub const WHITE: (u8, u8, u8) = (220, 215, 186);

pub fn apply(path: String) -> Project {
    let (language_enum, language) = get_git_language_of_path(Path::new(&path));

    let theme = Kanagawa::init(language_enum);

    Project {
        path,
        theme,
        language,
    }
}

