use crate::day::DayModule;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Habbit {
    pub title: String,
    pub is_complited: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TotalHabbits(pub Vec<Habbit>);
impl Habbit {
    pub fn new(title: String, is_complited: bool) -> Self {
        Self {
            title,
            is_complited,
        }
    }
}

impl DayModule for TotalHabbits {
    const ID: &'static str = "habits";
}
