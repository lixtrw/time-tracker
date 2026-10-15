use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, time::Duration};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalActivities {
    pub all: Vec<Activity>,
}
