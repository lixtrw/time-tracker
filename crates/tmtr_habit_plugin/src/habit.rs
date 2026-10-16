use tmtr_api::serde::{Serialize, Deserialize};


#[derive(Debug, Default, Serialize,Deserialize)]
#[serde(crate = "tmtr_api::serde")]
pub struct Habit{
    pub name: String,
    pub is_finished: bool
}
impl Habit{
    pub fn complete(&mut self){
        self.is_finished = true;
    }
}

#[derive(Debug, Default, Serialize,Deserialize)]
#[serde(crate = "tmtr_api::serde")]
pub struct TotalHabits(Vec<Habit>);
impl TotalHabits {
    pub fn add_habit(&mut self,habit: Habit){
        self.0.push(habit)
    }
}
