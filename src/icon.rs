use std::{process::Command, path::Path};
use std::{str, io};
use tui::{
    style::Color,
};

pub enum Language {
    Rust,
    TypeScript,
    Unknown,
}

pub type Icon = String;
type FilePath = String;

pub fn resolve_icon_and_color(path: String) -> (FilePath, Color, Icon) {
    let (icon, color) = match_icon_and_color(github_linguist(path.clone()).unwrap());

    (path, color, icon)
}


fn github_linguist(path: FilePath) -> Result<Language, io::Error> {
    // panic!("{}", path);

    if Path::new(&path).is_dir() {
        let output = Command::new("github-linguist").args([path.clone()]).output();

        match output {
            Ok(_) => {
                 match str::from_utf8(&output.unwrap().stdout) {
                     Ok(output) => {
                         if output.len() > 0 {
                            return Ok(match_language(output.split_whitespace().collect::<Vec<_>>()[2].to_string()))
                         } else {
                             return Ok(Language::Unknown)
                         }
                     },
                    Err(e) => return Ok(Language::Unknown),
                };
            },
            Err(e) => return Ok(Language::Unknown),
        }
    } else {
        Ok(Language::Unknown)
    }
}

fn match_language(language: String) -> Language {
    match language.as_str() {
        "Rust" => return Language::Rust,
        _ => return Language::Unknown,
    }
}

fn match_icon_and_color(language: Language) -> (Icon, Color) {
    match language {
        Language::Rust => return (" ".to_string(), Color::Red),
        _ => return ("".to_string(), Color::White)
    }
    
}
