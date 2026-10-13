use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Habbit {
    pub title: String,
    pub is_finished: bool,
}
impl Habbit {
    pub fn new(title: String, is_finished: bool) -> Self {
        Self { title, is_finished }
    }
}
