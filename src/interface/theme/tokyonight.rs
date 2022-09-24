use crate::backend::language::LanguageType;

use super::{Definition, Theme, WHITE};

pub struct TokyoNight;

impl Definition for TokyoNight {
    fn load(language: &LanguageType) -> Theme {
        match language {
            LanguageType::Rust => {
                return Theme {
                    icon: " ".to_string(),
                    color: (255, 158, 100),
                }
            }
            LanguageType::TypeScript => {
                return Theme {
                    icon: "ﯤ ".to_string(),
                    color: (61, 89, 161),
                }
            }
            LanguageType::JavaScript => {
                return Theme {
                    icon: " ".to_string(),
                    color: (224, 175, 104),
                }
            }
            LanguageType::Swift => {
                return Theme {
                    icon: "ﯣ ".to_string(),
                    color: (247, 118, 142),
                }
            }
            LanguageType::Elixir => {
                return Theme {
                    icon: " ".to_string(),
                    color: (157, 124, 216),
                }
            }
            LanguageType::Ruby => {
                return Theme {
                    icon: " ".to_string(),
                    color: (219, 75, 75),
                }
            }
            LanguageType::Markdown => {
                return Theme {
                    icon: " ".to_string(),
                    color: WHITE,
                }
            }
            LanguageType::HTML => {
                return Theme {
                    icon: " ".to_string(),
                    color: (178, 85, 91),
                }
            }
            LanguageType::Python => {
                return Theme {
                    icon: " ".to_string(),
                    color: (13, 185, 215),
                }
            }
            LanguageType::Java => {
                return Theme {
                    icon: " ".to_string(),
                    color: WHITE,
                }
            }
            LanguageType::EmacsLisp => {
                return Theme {
                    icon: " ".to_string(),
                    color: (187, 154, 247),
                }
            }
            LanguageType::Go => {
                return Theme {
                    icon: "ﳑ ".to_string(),
                    color: (125, 207, 255),
                }
            }
            LanguageType::Lua => {
                return Theme {
                    icon: " ".to_string(),
                    color: (61, 89, 161),
                }
            }
            LanguageType::CPlusPlus => {
                return Theme {
                    icon: " ".to_string(),
                    color: (255, 0, 124),
                }
            }
            LanguageType::CSS => {
                return Theme {
                    icon: " ".to_string(),
                    color: (61, 89, 161),
                }
            }
            LanguageType::Clojure => {
                return Theme {
                    icon: " ".to_string(),
                    color: (187, 154, 247),
                }
            }
            LanguageType::Vue => {
                return Theme {
                    icon: "﵂ ".to_string(),
                    color: (115, 218, 202),
                }
            }
            LanguageType::PHP => {
                return Theme {
                    icon: " ".to_string(),
                    color: (157, 124, 216),
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
}
