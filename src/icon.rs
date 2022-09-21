use std::{process::Command, path::Path};
use std::{str, io};
use tui::{
    style::Color,
};

enum Language {
    Rust,
    TypeScript,
    Unknown,
}

pub fn resolve_icon_and_color(path: String) -> (Color, String) {
    (Color::Red, github_linguist(path).unwrap())
}


fn github_linguist(path: String) -> Result<String, io::Error> {
    // panic!("{}", path);

    if Path::new(&path).is_dir() {
        let output = Command::new("github-linguist").args([path.clone()]).output();

        match output {
            Ok(_) => return Ok("command ran ".to_string()),
            Err(e) => return Ok("command did not run ".to_string()),
        }
    } else {
        Ok("its a file".to_string())
    }
}
