use std::{collections::BTreeSet, time::Duration};
use tmtr_api::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "tmtr_api::serde")]
pub struct Activity {
    pub tags: BTreeSet<String>,
    pub time_spent: Duration,
}

impl Activity {
    pub fn new(tags: Vec<String>, time_spent: Duration) -> Self {
        Self {
            tags: tags.into_iter().collect(),
            time_spent,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "tmtr_api::serde")]
pub struct TotalActivities(pub Vec<Activity>);

impl TotalActivities {
    pub fn add_activity(&mut self, activity: Activity) {
        self.0.push(activity);
    }
}
