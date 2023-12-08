use std::{collections::HashMap, iter::zip};

use crate::TaskCompleter;

pub struct Task8;

impl TaskCompleter for Task8 {
    fn get_name(&self) -> String {
        "8".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/day_08/input");
        let mut lines = contents.lines();
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        // Empty line
        lines.next();
        let mut mapping = HashMap::<&str, (&str, &str)>::new();
        for line in lines {
            mapping.insert(&line[0..3], (&line[7..10], &line[12..15]));
        }
        let mut space = "AAA";
        let mut count = 0;
        while space != "ZZZ" {
            let dir = instructions[count % instructions.len()];
            space = match dir {
                'L' => mapping[space].0,
                'R' => mapping[space].1,
                _ => panic!("Invalid character {}", dir),
            };
            count += 1;
        }
        count.to_string()
    }

    fn do_task_2(&self) -> String {
        todo!()
    }
}
