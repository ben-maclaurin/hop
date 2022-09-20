use std::path::PathBuf;

type IconWithPath = (String, String);

// The idea is to resolve the icons.
pub fn resolve(entries: Vec<PathBuf>) -> Vec<IconWithPath> {
    for entry in entries {
    }

    vec![("test".to_string(), "test".to_string())]
}
