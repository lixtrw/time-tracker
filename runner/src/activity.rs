use crate::day::DayModule;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, time::Duration};

use crate::activity;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Activity {
    pub tags: BTreeSet<String>,
    pub time_spent: Duration,
}
impl Activity {
    pub fn new(tags: Vec<String>, time_spent: Duration) -> Self {
        let tags_btree: BTreeSet<String> = tags.into_iter().collect();
        Self {
            tags: tags_btree,
            time_spent,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TotalActivities(pub Vec<Activity>);

impl TotalActivities {
    pub fn add_activity(&mut self, activity: Activity) {
        self.0.push(activity);
    }
}
impl DayModule for TotalActivities {
    const ID: &'static str = "activities";
}
