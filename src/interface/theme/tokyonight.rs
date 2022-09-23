use crate::backend::language::LanguageType;

use super::{Definition, Theme, WHITE};

pub struct TokyoNight;

impl Definition for TokyoNight {
    fn load(language: &LanguageType) -> Theme {
        match language {
            LanguageType::Rust => {
                return Theme {
                    icon: " ".to_string(),
                    color: (255, 150, 108),
                }
            }
            LanguageType::TypeScript => {
                return Theme {
                    icon: "ﯤ ".to_string(),
                    color: (62, 104, 215),
                }
            }
            LanguageType::JavaScript => {
                return Theme {
                    icon: " ".to_string(),
                    color: (255, 199, 119),
                }
            }
            LanguageType::Swift => {
                return Theme {
                    icon: "ﯣ ".to_string(),
                    color: (197, 59, 83),
                }
            }
            LanguageType::Elixir => {
                return Theme {
                    icon: " ".to_string(),
                    color: (192, 153, 255),
                }
            }
            LanguageType::Ruby => {
                return Theme {
                    icon: " ".to_string(),
                    color: (197, 59, 83),
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
                    color: (255, 117, 127),
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
                    color: (255, 0, 124),
                }
            }
            LanguageType::Go => {
                return Theme {
                    icon: "ﳑ ".to_string(),
                    color: (79, 214, 190),
                }
            }
            LanguageType::Lua => {
                return Theme {
                    icon: " ".to_string(),
                    color: (57, 75, 112),
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
                    color: (192, 153, 255),
                }
            }
            LanguageType::Clojure => {
                return Theme {
                    icon: " ".to_string(),
                    color: (255, 0, 124),
                }
            }
            LanguageType::Vue => {
                return Theme {
                    icon: "﵂ ".to_string(),
                    color: (79, 214, 190),
                }
            }
            LanguageType::PHP => {
                return Theme {
                    icon: " ".to_string(),
                    color: (192, 153, 255),
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
