use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Task {
    pub name: String,
    pub is_done: bool,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned(), is_done: false }
    }
}