mod activity;
mod day;
mod habbit;

use std::time::Duration;

use activity::*;

use crate::{
    day::{ActionType, DayData, DaysList},
    habbit::{Habbit, TotalHabbits},
};

fn main() {
    let format_text = |text_vec: Vec<&str>| {
        text_vec
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
    };

    let activity = Activity::new(
        format_text(vec!["english", "rust"]),
        Duration::from_secs(180),
    );
    let activity1 = Activity::new(format_text(vec!["math", "algebra"]), Duration::from_mins(5));

    let ta = TotalActivities(vec![activity, activity1]);

    let habbit = Habbit::new("English".into(), false);
    let th = TotalHabbits(vec![habbit]);

    let day = DayData {
        activities: ta,
        habbits: th,
    };
    let today = chrono::Local::now().date_naive();
    let mut today_data = DaysList::default();
    today_data.add_day(today, day);
    println!("{:#?}", today_data);
}
