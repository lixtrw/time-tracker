mod activity;
mod day;
mod habbit;
mod modules;

use color_eyre::Result;
use libloading::Library;
use std::time::Duration;
use tmtr_plugin_api::Plugin;

use crate::{
    activity::{Activity, TotalActivities},
    day::DaysList,
    habbit::{Habbit, TotalHabbits},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    test3()?;
    Ok(())
}

fn init_plugins() -> Result<Vec<Box<dyn Plugin>>> {
    let mut modules: Vec<Box<dyn Plugin>> = Vec::new();

    #[cfg(feature = "buildin_activity")]
    modules.push(tmtr_activity_plugin::init());

    Ok(modules)
}

#[cfg(feature = "debug")]
fn test3() -> Result<()> {
    let plugins = init_plugins()?;
    for i in plugins {
        println!("{}", i.id());
        println!("{:?}", i.commands());
    }
    Ok(())
}

#[cfg(feature = "debug")]
fn test2() -> Result<()> {
    unsafe {
        use libloading::Symbol;
        use tmtr_plugin_api::Plugin;

        let lib = Library::new(
            "/home/lixtr/Documents/rust/tmtr/crates/tmtr_activties_plugin/target/release/libtmtr_activties_plugin.so",
        )?;
        let symb: Symbol<fn() -> Box<dyn Plugin>> = lib.get(b"init_plugin")?;
        let plugin = symb();
        println!("{:?}", plugin.commands());
    }
    Ok(())
}

#[cfg(feature = "debug")]
fn test1() -> Result<()> {
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
