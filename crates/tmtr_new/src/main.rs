mod activity;
mod day;
mod habbit;

use color_eyre::Result;
use std::time::Duration;

use crate::{
    activity::{Activity, TotalActivities},
    day::DaysList,
    habbit::{Habbit, TotalHabbits},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let format_format_tags = |v: Vec<&str>| v.into_iter().map(String::from).collect::<Vec<_>>();

    let activity = Activity::new(
        format_format_tags(vec!["english", "rust"]),
        Duration::from_secs(180),
    );
    let activity1 = Activity::new(
        format_format_tags(vec!["math", "algebra"]),
        Duration::from_secs(300),
    );
    let habbit = Habbit::new("English".into(), false);

    let ta = TotalActivities(vec![activity1, activity]);
    let th = TotalHabbits(vec![habbit]);

    let today = chrono::Local::now().date_naive();
    let mut list = DaysList::default();

    let today_data = list.day_mut(today);
    today_data.add_data::<TotalActivities>(&ta)?;
    today_data.add_data::<TotalHabbits>(&th)?;

    let today_activity_data = list.day_mut(today).get_data::<TotalActivities>();
    let today_data_string = serde_json::to_string_pretty(list.day_mut(today))?;

    println!("{:#?}", &list);

    println!("-----------");
    println!("{:#?}", today_activity_data);

    println!("-----------");
    println!("{}", today_data_string);

    Ok(())
}
