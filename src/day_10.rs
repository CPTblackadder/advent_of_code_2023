use crate::TaskCompleter;


pub struct Task10;

impl TaskCompleter for Task10 {
    fn get_name(&self) -> String {
        "10".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/day_10/input");


        "todo".to_owned()
    }

    fn do_task_2(&self) -> String {
        "todo".to_owned()
    }
}
