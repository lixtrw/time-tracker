use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{activity::TotalActivities, habbit::TotalHabbits};

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionType {
    Activities(TotalActivities),
    Habbits(TotalHabbits),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayData {
    pub data: Vec<ActionType>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DaysList {
    pub days: BTreeMap<NaiveDate, DayData>,
}
impl DaysList {
    pub fn add_day(&mut self, date: NaiveDate, day_data: DayData) {
        self.days.insert(date, day_data);
    }
}
