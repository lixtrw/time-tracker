use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    activity::{self, Activity, TotalActivities},
    habbit::TotalHabbits,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionType {
    Activities(TotalActivities),
    Habbits(TotalHabbits),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayData {
    pub activities: TotalActivities,
    pub habbits: TotalHabbits,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DaysList {
    pub days: BTreeMap<NaiveDate, DayData>,
}
impl DaysList {
    pub fn add_day(&mut self, date: NaiveDate, day_data: DayData) {
        self.days.insert(date, day_data);
    }
    pub fn get_day_activities(&self, date: NaiveDate) -> Option<&TotalActivities> {
        if let Some(day_data) = self.days.get(&date) {
            Some(&day_data.activities)
        } else {
            None
        }
    }
}
