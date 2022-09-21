use std::{io, str};
use std::{path::Path, process::Command};
use tui::style::Color;

pub enum Language {
    Rust,
    TypeScript,
    JavaScript,
    Swift,
    Elixir,
    Ruby,
    Markdown,
    HTML,
    Python,
    Java,
    EmacsLisp,
    Go,
    Unknown,
}

pub type Icon = String;
type FilePath = String;

pub fn apply(path: String) -> (FilePath, Color, Icon) {
    let (icon, color) = apply_theme(resolve_language(path.clone()).unwrap());

    (path, color, icon)
}

fn resolve_language(path: FilePath) -> Result<Language, io::Error> {
    if Path::new(&path).is_dir() {
        let output = Command::new("github-linguist")
            .args([path.clone()])
            .output();

        match output {
            Ok(_) => {
                match str::from_utf8(&output.unwrap().stdout) {
                    Ok(output) => {
                        if output.len() > 0 {
                            return Ok(match_language(
                                output.split_whitespace().collect::<Vec<_>>()[2].to_string(),
                            ));
                        } else {
                            return Ok(Language::Unknown);
                        }
                    }
                    Err(_) => return Ok(Language::Unknown),
                };
            }
            Err(_) => return Ok(Language::Unknown),
        }
    } else {
        Ok(Language::Unknown)
    }
}

fn match_language(language: String) -> Language {
    match language.as_str() {
        "Rust" => return Language::Rust,
        "TypeScript" => return Language::TypeScript,
        "JavaScript" => return Language::JavaScript,
        "Swift" => return Language::Swift,
        "Elixir" => return Language::Elixir,
        "Ruby" => return Language::Ruby,
        "Markdown" => return Language::Markdown,
        "HTML" => return Language::HTML,
        "Python" => return Language::Python,
        "Java" => return Language::Java,
        "Emacs" => return Language::EmacsLisp,
        "Go" => return Language::Go,
        _ => return Language::Unknown,
    }
}

fn apply_theme(language: Language) -> (Icon, Color) {
    match language {
        Language::Rust => return (" ".to_string(), Color::Rgb(255, 160, 102)),
        Language::TypeScript => return ("ﯤ ".to_string(), Color::Rgb(126, 156, 216)),
        Language::JavaScript => return (" ".to_string(), Color::Rgb(220, 165, 97)),
        Language::Swift => return ("ﯣ ".to_string(), Color::Rgb(255, 93, 98)),
        Language::Elixir => return (" ".to_string(), Color::Rgb(149, 127, 184)),
        Language::Ruby => return (" ".to_string(), Color::Rgb(195, 64, 67)),
        Language::Markdown => return (" ".to_string(), Color::White),
        Language::HTML => return (" ".to_string(), Color::Rgb(228, 104, 118)),
        Language::Python => return (" ".to_string(), Color::Rgb(156, 171, 202)),
        Language::Java => return (" ".to_string(), Color::Rgb(147, 128, 86)),
        Language::EmacsLisp => return (" ".to_string(), Color::Rgb(210, 126, 153)),
        Language::Go => return ("ﳑ ".to_string(), Color::Rgb(163, 212, 213)),
        _ => return (" ".to_string(), Color::White),
    }
}
