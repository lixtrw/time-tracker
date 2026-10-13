use serde::{Deserialize, Serialize};

use crate::{
    activity::{self, Activity},
    habbit_tracker::Habbit,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionType {
    Activity(Activity),
    Habbit(Habbit),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub actions: Vec<ActionType>,
}
impl Day {
    pub fn new(data: Vec<ActionType>) -> Self {
        Self { actions: data }
    }
}
