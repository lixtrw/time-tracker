use clap::{Parser, Subcommand};
use color_eyre::Result;
use tmtr_api::Plugin;

fn main() -> Result<()> {
    color_eyre::install()?;
    test3()?;
    Ok(())
}

fn init_plugins() -> Result<Vec<Box<dyn Plugin>>> {
    let mut modules: Vec<Box<dyn Plugin>> = Vec::new();

    #[cfg(feature = "buildin_activity")]
    modules.push(tmtr_activity_plugin::init());

    #[cfg(feature = "buildin_habit")]
    modules.push(tmtr_habit_plugin::init());

    Ok(modules)
}

#[cfg(feature = "debug")]
fn test3() -> Result<()> {
    use chrono::{Local, NaiveDate};
    use tmtr_api::day::DaysList;

    let mut plugins = init_plugins()?;
    let today = Local::now().date_naive();
    let mut day_list = DaysList::default();
    let today_data = day_list.day_mut(today);
    let today_activities = today_data.get_data_mut("activity");

    for p in &plugins {
        println!("{} v{}", p.id(), p.version());
        println!("{:?}", p.commands());
    }
    let anct_id = plugins.iter_mut().find(|f| f.id() == "activity");
    if let Some(a) = anct_id {
        println!("{:?}", a.id());
        a.execute(
            "add".into(),
            &vec!["eng".into(), "rust".into(), "-t".into(), "60".into()],
            today_activities,
        );
    }

    println!("{:#?}", day_list);

    Ok(())
}
