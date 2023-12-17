use crate::TaskCompleter;
use pathfinding::prelude::astar;

pub struct Task17;

impl TaskCompleter for Task17 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_17/input");
        let chars = Grid::from_string_i64(contents);

        let res = astar(Coord());

        "todo".to_owned()
    }

    fn do_task_2(&self) -> String {
        "todo".to_owned()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
