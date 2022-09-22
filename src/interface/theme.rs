use std::path::Path;

use crate::backend::linguist::{get_git_language_of_path, Language};

pub type Icon = String;
pub type Theme = (Icon, (u8, u8, u8));

pub const WHITE: (u8, u8, u8) = (220, 215, 186);

pub fn apply(path: String) -> (String, Theme) {
    let language = get_git_language_of_path(Path::new(&path));
    (path, kanagawa(language))
}

fn kanagawa(language: Language) -> Theme {
    match language {
        Language::Rust => return (" ".to_string(), (255, 160, 102)),
        Language::TypeScript => return ("ﯤ ".to_string(), (126, 156, 216)),
        Language::JavaScript => return (" ".to_string(), (220, 165, 97)),
        Language::Swift => return ("ﯣ ".to_string(), (255, 93, 98)),
        Language::Elixir => return (" ".to_string(), (149, 127, 184)),
        Language::Ruby => return (" ".to_string(), (195, 64, 67)),
        Language::Markdown => return (" ".to_string(), WHITE),
        Language::HTML => return (" ".to_string(), (228, 104, 118)),
        Language::Python => return (" ".to_string(), (156, 171, 202)),
        Language::Java => return (" ".to_string(), (147, 128, 86)),
        Language::EmacsLisp => return (" ".to_string(), (210, 126, 153)),
        Language::Go => return ("ﳑ ".to_string(), (163, 212, 213)),
        _ => return (" ".to_string(), WHITE),
    }
}
