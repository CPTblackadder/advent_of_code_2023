use crate::TaskCompleter;



pub struct Task14;

impl TaskCompleter for Task14 {
    fn get_name(&self) -> String {
        "14".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_14/input");
        


        "Todo".to_owned()
    }

    fn do_task_2(&self) -> String {
        "Todo".to_owned()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
