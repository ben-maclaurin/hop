use std::path::Path;

use crate::backend::{
    linguist::{get_git_language_of_path, Language},
    project::Project,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Theme {
    pub icon: String,
    pub color: (u8, u8, u8),
}

pub const WHITE: (u8, u8, u8) = (220, 215, 186);

pub fn apply(path: String) -> Project {
    let (language_enum, language) = get_git_language_of_path(Path::new(&path));

    Project {
        path,
        theme: kanagawa(language_enum),
        language,
    }
}

fn kanagawa(language: Language) -> Theme {
    match language {
        Language::Rust => {
            return Theme {
                icon: " ".to_string(),
                color: (255, 160, 102),
            }
        }
        Language::TypeScript => {
            return Theme {
                icon: "ﯤ ".to_string(),
                color: (126, 156, 216),
            }
        }
        Language::JavaScript => {
            return Theme {
                icon: " ".to_string(),
                color: (220, 165, 97),
            }
        }
        Language::Swift => {
            return Theme {
                icon: "ﯣ ".to_string(),
                color: (255, 93, 98),
            }
        }
        Language::Elixir => {
            return Theme {
                icon: " ".to_string(),
                color: (149, 127, 184),
            }
        }
        Language::Ruby => {
            return Theme {
                icon: " ".to_string(),
                color: (195, 64, 67),
            }
        }
        Language::Markdown => {
            return Theme {
                icon: " ".to_string(),
                color: WHITE,
            }
        }
        Language::HTML => {
            return Theme {
                icon: " ".to_string(),
                color: (228, 104, 118),
            }
        }
        Language::Python => {
            return Theme {
                icon: " ".to_string(),
                color: (156, 171, 202),
            }
        }
        Language::Java => {
            return Theme {
                icon: " ".to_string(),
                color: (147, 128, 86),
            }
        }
        Language::EmacsLisp => {
            return Theme {
                icon: " ".to_string(),
                color: (210, 126, 153),
            }
        }
        Language::Go => {
            return Theme {
                icon: "ﳑ ".to_string(),
                color: (163, 212, 213),
            }
        }
        Language::Lua => {
            return Theme {
                icon: " ".to_string(),
                color: (126, 156, 216),
            }
        }
        Language::CPlusPlus => {
            return Theme {
                icon: " ".to_string(),
                color: (210, 126, 153),
            }
        }
        Language::CSS => {
            return Theme {
                icon: " ".to_string(),
                color: (147, 138, 169),
            }
        }
        _ => {
            return Theme {
                icon: " ".to_string(),
                color: WHITE,
            }
        }
    }
}
