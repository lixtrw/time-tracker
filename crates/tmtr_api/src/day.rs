use chrono::NaiveDate;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct DayData(BTreeMap<String, Value>);

impl DayData {
    pub fn get_data_mut(&mut self, plugin_id: &str) -> &mut Value {
        self.0.entry(plugin_id.into()).or_insert(Value::Null)
    }
    pub fn get_data(&self, plugin_id: &str) -> Option<&Value> {
        self.0.get(plugin_id)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DaysList {
    pub days: BTreeMap<NaiveDate, DayData>,
}

impl DaysList {
    pub fn day_mut(&mut self, date: NaiveDate) -> &mut DayData {
        self.days.entry(date).or_default()
    }
}
