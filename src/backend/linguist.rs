use std::{path::Path, process::Command, process::Output, str::from_utf8};

use super::project::Empty;

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
    Lua,
    CPlusPlus,
    CSS,
    Clojure,
    Vue,
    PHP,
    Unspecified,
}

impl Language {}

// impl Empty for Language {
//     fn empty() -> Self {
//     }
// }

pub fn get_git_language_of_path(path: &Path) -> (Language, String) {
    if path.is_dir() {
        let linguist_output = Command::new("github-linguist")
            .args([path])
            .output()
            .expect("github-linguist is not installed or is broken");

        return parse_language(linguist_output);
    }

    (Language::Unspecified, "".to_string())
}

fn parse_language(output: Output) -> (Language, String) {
    let output = from_utf8(&output.stdout).unwrap();

    if output.len() > 0 {
        let language_index = 2;

        let language = output.split_whitespace().collect::<Vec<_>>()[language_index];

        match language {
            "Rust" => return (Language::Rust, "rust".to_string()),
            "TypeScript" => return (Language::TypeScript, "typescript".to_string()),
            "JavaScript" => return (Language::JavaScript, "javascript".to_string()),
            "Swift" => return (Language::Swift, "swift".to_string()),
            "Elixir" => return (Language::Elixir, "elixir".to_string()),
            "Ruby" => return (Language::Ruby, "ruby".to_string()),
            "Markdown" => return (Language::Markdown, "markdown".to_string()),
            "HTML" => return (Language::HTML, "html".to_string()),
            "Python" => return (Language::Python, "python".to_string()),
            "Java" => return (Language::Java, "java".to_string()),
            "Emacs" => return (Language::EmacsLisp, "emacslisp".to_string()),
            "Go" => return (Language::Go, "go".to_string()),
            "Lua" => return (Language::Lua, "lua".to_string()),
            "C++" => return (Language::CPlusPlus, "c++".to_string()),
            "CSS" => return (Language::CSS, "css".to_string()),
            "Clojure" => return (Language::Clojure, "clojure".to_string()),
            "Vue" => return (Language::Vue, "vue".to_string()),
            "PHP" => return (Language::PHP, "php".to_string()),
            _ => return (Language::Unspecified, "".to_string()),
        }
    }

    (Language::Unspecified, "".to_string())
}
