use crate::TaskCompleter;

pub struct Task11;

impl TaskCompleter for Task11 {
    fn get_name(&self) -> String {
        "11".to_owned()
    }

    fn do_task_1(&self) -> String {
        "todo!()".to_owned()
    }

    fn do_task_2(&self) -> String {
        "todo!()".to_owned()
    }
}
