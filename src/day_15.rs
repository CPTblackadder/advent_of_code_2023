use crate::TaskCompleter;

fn hash(input: &str) -> u64 {
    input.chars().fold(0, |v, c| ((v + (c as u64)) * 17) % 256)
}

pub struct Task15;

impl TaskCompleter for Task15 {
    fn get_name(&self) -> String {
        "15".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_15/input");
        contents.split(",").map(hash).sum::<u64>().to_string()
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

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{day_08::str_to_index, day_15::hash};

    #[test]
    fn hash_test() {
        assert_eq!(hash("HASH"), 52);
    }
}
