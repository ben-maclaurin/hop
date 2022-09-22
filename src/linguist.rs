// fn resolve_language(path: FilePath) -> Result<Language, io::Error> {
//     if Path::new(&path).is_dir() {
//         let output = Command::new("github-linguist")
//             .args([path.clone()])
//             .output();

//         match output {
//             Ok(_) => {
//                 match str::from_utf8(&output.unwrap().stdout) {
//                     Ok(output) => {
//                         if output.len() > 0 {
//                             return Ok(match_language(
//                                 output.split_whitespace().collect::<Vec<_>>()[2].to_string(),
//                             ));
//                         } else {
//                             return Ok(Language::Unknown);
//                         }
//                     }
//                     Err(_) => return Ok(Language::Unknown),
//                 };
//             }
//             Err(_) => return Ok(Language::Unknown),
//         }
//     } else {
//         Ok(Language::Unknown)
//     }
// }
use std::{path::Path, process::Command, process::Output, str::from_utf8};

enum Language {
    Rust,
    Unspecified,
}

fn get_git_language_of_path(path: &Path) -> Language {
    // path is IMMMUTABLE ref.
    if path.is_dir() {
        let linguist_output = Command::new("github-linguist")
            .args([path])
            .output()
            .expect("github-linguist is not installed or is broken");

        return parse_language(linguist_output); // linguist_output is MOVED!
    }

    Language::Unspecified //this is temp to fix error
} // all values DROPPED here (path drops here)

fn parse_language(output: Output) -> Language {
    let output = from_utf8(&output.stdout).unwrap(); // a string reference.

    if output.len() > 0 {
        let language_index = 2;

        let language = output.split_whitespace().collect::<Vec<_>>()[language_index];

        match language {
            "Rust" => return Language::Rust,
            _ => return Language::Unspecified,
        }
    } 

    Language::Unspecified
}

