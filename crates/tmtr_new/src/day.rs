use chrono::NaiveDate;
use color_eyre::Result;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::BTreeMap;

pub trait DayModule: Default + Serialize + DeserializeOwned {
    const ID: &'static str;
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct DayData(BTreeMap<String, Value>);

impl DayData {
    pub fn add_data<Module: DayModule>(&mut self, data: &Module) -> Result<()> {
        self.0
            .insert(Module::ID.to_string(), serde_json::to_value(data)?);
        Ok(())
    }
    pub fn get_data<Module: DayModule>(&self) -> Result<Module> {
        if let Some(module) = self.0.get(Module::ID) {
            Ok(serde_json::from_value(module.clone())?)
        } else {
            Err(eyre!("module noit found"))
        }
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
