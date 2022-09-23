use std::{path::Path, process::Command, process::Output, str::from_utf8};

use serde::{Deserialize, Serialize};

use super::project::Empty;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LanguageType {
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Language {
    pub language: LanguageType,
    pub query: String,
}

impl Empty for Language {
    fn empty() -> Self {
        Self {
            language: LanguageType::Unspecified,
            query: "".to_string(),
        }
    }
}

impl Language {
    fn set(mut self, output: Output) -> Self {
        let output = from_utf8(&output.stdout).unwrap();

        if output.len() > 0 {
            let language_index = 2;

            let language = output.split_whitespace().collect::<Vec<_>>()[language_index];

            match language {
                "Rust" => {
                    self = Language {
                        language: LanguageType::Rust,
                        query: "rust".to_string(),
                    }
                }
                "TypeScript" => {
                    self = Language {
                        language: LanguageType::TypeScript,
                        query: "typescript".to_string(),
                    }
                }
                "JavaScript" => {
                    self = Language {
                        language: LanguageType::JavaScript,
                        query: "javascript".to_string(),
                    }
                }
                "Swift" => {
                    self = Language {
                        language: LanguageType::Swift,
                        query: "swift".to_string(),
                    }
                }
                "Elixir" => {
                    self = Language {
                        language: LanguageType::Elixir,
                        query: "elixir".to_string(),
                    }
                }
                "Ruby" => {
                    self = Language {
                        language: LanguageType::Ruby,
                        query: "ruby".to_string(),
                    }
                }
                "Markdown" => {
                    self = Language {
                        language: LanguageType::Markdown,
                        query: "markdown".to_string(),
                    }
                }
                "HTML" => {
                    self = Language {
                        language: LanguageType::HTML,
                        query: "html".to_string(),
                    }
                }
                "Python" => {
                    self = Language {
                        language: LanguageType::Python,
                        query: "python".to_string(),
                    }
                }
                "Java" => {
                    self = Language {
                        language: LanguageType::Java,
                        query: "java".to_string(),
                    }
                }
                "Emacs" => {
                    self = Language {
                        language: LanguageType::EmacsLisp,
                        query: "emacslisp".to_string(),
                    }
                }
                "Go" => {
                    self = Language {
                        language: LanguageType::Go,
                        query: "go".to_string(),
                    }
                }
                "Lua" => {
                    self = Language {
                        language: LanguageType::Lua,
                        query: "lua".to_string(),
                    }
                }
                "C++" => {
                    self = Language {
                        language: LanguageType::CPlusPlus,
                        query: "c++".to_string(),
                    }
                }
                "CSS" => {
                    self = Language {
                        language: LanguageType::CSS,
                        query: "css".to_string(),
                    }
                }
                "Clojure" => {
                    self = Language {
                        language: LanguageType::Clojure,
                        query: "clojure".to_string(),
                    }
                }
                "Vue" => {
                    self = Language {
                        language: LanguageType::Vue,
                        query: "vue".to_string(),
                    }
                }
                "PHP" => {
                    self = Language {
                        language: LanguageType::PHP,
                        query: "php".to_string(),
                    }
                }
                _ => self = Language::empty(),
            }
        }

        self
    }
}

pub fn detect_language(path: &Path) -> Language {
    if path.is_dir() {
        let linguist_output = Command::new("github-linguist")
            .args([path])
            .output()
            .expect("github-linguist is not installed or is broken");

        return Language::empty().set(linguist_output);
    }

    Language::empty()
}
