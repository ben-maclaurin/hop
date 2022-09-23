use crate::backend::linguist::Language;

use super::{Definition, Theme, WHITE};

pub struct Kanagawa;

impl Definition for Kanagawa {
    fn load(language: Language) -> Theme {
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
            Language::Clojure => {
                return Theme {
                    icon: " ".to_string(),
                    color: (228, 104, 118),
                }
            }
            Language::Vue => {
                return Theme {
                    icon: "﵂ ".to_string(),
                    color: (152, 187, 108),
                }
            }
            Language::PHP => {
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
