use std::path::Path;

use crate::backend::{
    language::{detect_language, LanguageType},
    project::{Empty, Project},
};
use serde_derive::{Deserialize, Serialize};

pub mod kanagawa;
pub mod tokyonight;
use tokyonight::TokyoNight;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Theme {
    pub icon: String,
    pub color: (u8, u8, u8),
}

pub trait Definition {
    fn load(lanauge: &LanguageType) -> Theme;
}

pub const WHITE: (u8, u8, u8) = (192, 202, 245);

impl Empty for Theme {
    fn empty() -> Self {
        Self {
            icon: " ".to_string(),
            color: WHITE,
        }
    }
}

pub fn apply(path: String) -> Project {
    let language = detect_language(Path::new(&path));

    let theme = TokyoNight::load(&language.language);

    Project {
        path,
        theme,
        language,
    }
}
