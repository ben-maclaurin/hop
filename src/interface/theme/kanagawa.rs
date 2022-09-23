use crate::backend::language::LanguageType;

use super::{Definition, Theme, WHITE};

pub struct Kanagawa;

impl Definition for Kanagawa {
    fn load(language: &LanguageType) -> Theme {
        match language {
            LanguageType::Rust => {
                return Theme {
                    icon: " ".to_string(),
                    color: (255, 160, 102),
                }
            }
            LanguageType::TypeScript => {
                return Theme {
                    icon: "ﯤ ".to_string(),
                    color: (126, 156, 216),
                }
            }
            LanguageType::JavaScript => {
                return Theme {
                    icon: " ".to_string(),
                    color: (220, 165, 97),
                }
            }
            LanguageType::Swift => {
                return Theme {
                    icon: "ﯣ ".to_string(),
                    color: (255, 93, 98),
                }
            }
            LanguageType::Elixir => {
                return Theme {
                    icon: " ".to_string(),
                    color: (149, 127, 184),
                }
            }
            LanguageType::Ruby => {
                return Theme {
                    icon: " ".to_string(),
                    color: (195, 64, 67),
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
                    color: (228, 104, 118),
                }
            }
            LanguageType::Python => {
                return Theme {
                    icon: " ".to_string(),
                    color: (156, 171, 202),
                }
            }
            LanguageType::Java => {
                return Theme {
                    icon: " ".to_string(),
                    color: (147, 128, 86),
                }
            }
            LanguageType::EmacsLisp => {
                return Theme {
                    icon: " ".to_string(),
                    color: (210, 126, 153),
                }
            }
            LanguageType::Go => {
                return Theme {
                    icon: "ﳑ ".to_string(),
                    color: (163, 212, 213),
                }
            }
            LanguageType::Lua => {
                return Theme {
                    icon: " ".to_string(),
                    color: (126, 156, 216),
                }
            }
            LanguageType::CPlusPlus => {
                return Theme {
                    icon: " ".to_string(),
                    color: (210, 126, 153),
                }
            }
            LanguageType::CSS => {
                return Theme {
                    icon: " ".to_string(),
                    color: (147, 138, 169),
                }
            }
            LanguageType::Clojure => {
                return Theme {
                    icon: " ".to_string(),
                    color: (228, 104, 118),
                }
            }
            LanguageType::Vue => {
                return Theme {
                    icon: "﵂ ".to_string(),
                    color: (152, 187, 108),
                }
            }
            LanguageType::PHP => {
                return Theme {
                    icon: " ".to_string(),
                    color: (149, 127, 184),
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
