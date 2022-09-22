use serde_derive::{Deserialize, Serialize};

use crate::interface::theme::Theme;

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    path: String,
    theme: Theme,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Store {
    projects: Vec<Project>
}

fn store() {

}
