use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub tags: Vec<String>,
    pub time_spent: Duration,
}
impl Activity {
    pub fn new(tags: Vec<String>, time_spent: Duration) -> Self {
        Self { tags, time_spent }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    pub activities: Vec<Activity>,
    pub date: NaiveDate,
}
impl Day {
    pub fn new(activities: Vec<Activity>, date: NaiveDate) -> Self {
        Self { activities, date }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct All {
    pub days: Vec<Day>,
}
impl All {
    pub fn get_day(&self, date: NaiveDate) -> Option<&Day> {
        self.days.iter().find(|day| day.date == date)
    }
    pub fn add_actitity(&mut self) {}
}
