#![allow(dead_code)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Day {
    pub activities: Vec<Activity>,
}

impl Day {
    pub fn new(activities: Vec<Activity>) -> Self {
        Self { activities }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct All {
    pub days: HashMap<NaiveDate, Day>,
}

impl All {
    pub fn get_day(&self, date: NaiveDate) -> Option<&Day> {
        self.days.get(&date)
    }
    pub fn get_day_mut(&mut self, date: NaiveDate) -> Option<&mut Day> {
        self.days.get_mut(&date)
    }
    pub fn add_activity(&mut self, date: NaiveDate, activity: Activity) {
        self.days.entry(date).or_default().activities.push(activity);
    }
    pub fn remove_activity_by_index(&mut self, date: NaiveDate, index: usize) -> Option<Activity> {
        let day = self.get_day_mut(date)?;

        if index < day.activities.len() {
            Some(day.activities.remove(index))
        } else {
            None
        }
    }
    pub fn get_period(&self, since: NaiveDate, until: NaiveDate) -> Vec<(NaiveDate, Day)> {
        let mut result = Vec::new();
        let mut date = since;
        while date <= until {
            result.push((date, self.days.get(&date).cloned().unwrap_or_default()));
            if date == until {
                break;
            }
            date += chrono::Duration::days(1);
        }
        result
    }
    pub fn get_tags(&self, tags: &[String]) -> Duration {
        let tags: BTreeSet<String> = tags.iter().cloned().collect();

        self.days
            .values()
            .flat_map(|day| &day.activities)
            .filter(|a| tags.is_subset(&a.tags))
            .map(|a| a.time_spent)
            .sum()
    }
}

