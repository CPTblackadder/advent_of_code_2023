use std::{
    iter::zip,
    ops::{Mul, Sub},
};

use crate::helpers::get_contents;

fn get_distance_for_given_charge<T>(total_time: T, charging_time: T) -> T
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    let speed = charging_time;
    let time_travelling = total_time - charging_time;
    let distance = speed * time_travelling;
    distance
}

pub fn run_task_1() {
    let contents = get_contents("six".to_owned());
    let mut lines = contents.lines();
    let times: &Vec<i32> = &lines.next().unwrap()[12..]
        .split("   ")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    let distances: &Vec<i32> = &lines.next().unwrap()[12..]
        .split("   ")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let result = zip(times, distances)
        .map(|(time, record_distance)| {
            (0..*time)
                .map(|charging_time| get_distance_for_given_charge(*time, charging_time))
                .filter(|distance| distance > record_distance)
                .count()
        })
        .fold(1, |x, y| x * y);

    println!("Result is: {}", result);
}

pub fn run_task_2() {
    let contents = get_contents("six".to_owned());
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

    let result = (0..*time)
        .map(|charging_time| get_distance_for_given_charge(*time, charging_time))
        .filter(|distance| distance > record_distance)
        .count();

    println!("Result is: {}", result);
}
