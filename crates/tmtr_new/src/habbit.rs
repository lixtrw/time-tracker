use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Habbit {
    pub title: String,
    pub is_complited: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalHabbits {
    pub all: Vec<Habbit>,
}
impl Habbit {
    pub fn new(title: String, is_complited: bool) -> Self {
        Self {
            title,
            is_complited,
        }
    }
}
