use std::ops::Range;

use crate::TaskCompleter;

pub struct Task22;

struct Brick {
    start: (i64, i64, i64),
    end: (i64, i64, i64),
}

impl Brick {
    
}



impl TaskCompleter for Task22 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_22/input");


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
