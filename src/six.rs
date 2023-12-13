use std::{
    iter::zip,
    ops::{Mul, Sub},
};

use crate::TaskCompleter;

fn get_distance_for_given_charge<T>(total_time: T, charging_time: T) -> T
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    let speed = charging_time;
    let time_travelling = total_time - charging_time;
    let distance = speed * time_travelling;
    distance
}

pub struct Task6;

impl TaskCompleter for Task6 {
    fn get_name(&self) -> String {
        "6".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/six/input");
        let mut lines = contents.lines();
        let times: &Vec<i32> = &lines.next().unwrap()[12..]
            .split("   ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        let distances: &Vec<i32> = &lines.next().unwrap()[12..]
            .split("   ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        zip(times, distances)
            .map(|(time, record_distance)| {
                (0..*time)
                    .map(|charging_time| get_distance_for_given_charge(*time, charging_time))
                    .filter(|distance| distance > record_distance)
                    .count()
            })
            .fold(1, |x, y| x * y)
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents = include_str!("../input/six/input");
        let mut lines = contents.lines();
        let time = &lines.next().unwrap()[12..]
            .split("   ")
            .map(|x| x.trim())
            .fold(String::new(), |x, y| x + y)
            .parse::<u64>()
            .unwrap();
        let record_distance = &lines.next().unwrap()[12..]
            .split("   ")
            .map(|x| x.trim())
            .fold(String::new(), |x, y| x + y)
            .parse::<u64>()
            .unwrap();

        (0..*time)
            .map(|charging_time| get_distance_for_given_charge(*time, charging_time))
            .filter(|distance| distance > record_distance)
            .count()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("114400".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("21039729".to_owned())
    }
}
