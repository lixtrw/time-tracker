use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    activity::{self, Activity, AllActivities},
    habbit_tracker::{AllHabbits, Habbit},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionType {
    Activities(AllActivities),
    Habbits(AllHabbits),
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DayList {
    pub days: HashMap<NaiveDate, Day>,
}
impl DayList {
    pub fn add_day(&mut self, date: NaiveDate, day_data: Day) {
        self.days.insert(date, day_data);
    }
}
