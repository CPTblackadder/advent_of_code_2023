use crate::TaskCompleter;

pub struct Task20;

impl TaskCompleter for Task20 {
    fn do_task_1(&self) -> String {
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