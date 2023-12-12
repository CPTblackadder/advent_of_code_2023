use crate::TaskCompleter;

struct Task12;

impl TaskCompleter for Task12 {
    fn get_name(&self) -> String {
        "12".to_owned()
    }

    fn do_task_1(&self) -> String {
        "todo!()".to_owned()
    }

    fn do_task_2(&self) -> String {
        "todo!()".to_owned()
    }
}
