use std::{path::Path, process::Command, process::Output, str::from_utf8};

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
    Unspecified,
}

pub fn get_git_language_of_path(path: &Path) -> Language {
    if path.is_dir() {
        let linguist_output = Command::new("github-linguist")
            .args([path])
            .output()
            .expect("github-linguist is not installed or is broken");

        return parse_language(linguist_output);
    }

    Language::Unspecified
}

fn parse_language(output: Output) -> Language {
    let output = from_utf8(&output.stdout).unwrap();

    if output.len() > 0 {
        let language_index = 2;

        let language = output.split_whitespace().collect::<Vec<_>>()[language_index];

        match language {
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
            _ => return Language::Unspecified,
        }
    }

    Language::Unspecified
}
