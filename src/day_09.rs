use std::{iter, ops::Sub};

use crate::TaskCompleter;

pub struct Task9;

fn reduce_line<T>(i: &Vec<T>) -> Vec<T>
where
    T: Sub<Output = T> + Clone + Copy,
{
    let mut value = i[0].clone();
    i[1..]
        .iter()
        .map(|x| {
            let v = value;
            value = *x;
            *x - v
        })
        .collect()
}

impl TaskCompleter for Task9 {
    fn get_name(&self) -> String {
        "9".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/day_09/input");
        let lines = contents.lines();

        lines
            .map(|x| {
                let mut i: Vec<i32> = x.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
                let mut last_items = vec![];
                while !i.iter().all(|x| x == &0) {
                    last_items.push(*i.last().unwrap());
                    i = reduce_line(&i);
                }
                last_items.reverse();
                let mut value = 0;
                for i in last_items {
                    value = i + value;
                }
                value
            })
            .sum::<i32>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents = include_str!("../input/day_09/input");
        let lines = contents.lines();

        lines
            .map(|x| {
                let mut i: Vec<i32> = x.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
                let mut last_items = vec![];
                while !i.iter().all(|x| x == &0) {
                    last_items.push(*i.first().unwrap());
                    i = reduce_line(&i);
                }
                last_items.reverse();
                let mut value = 0;
                for i in last_items {
                    value = i - value;
                }
                value
            })
            .sum::<i32>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("1980437560".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("977".to_owned())
    }
}
