use std::{
    fmt::Display,
    iter::{once, zip},
    sync::Mutex,
    time::{Duration, Instant},
};

use five::Task5;
use four::Task4;
use seven::Task7;
use six::Task6;
use three::Task3;

use crate::{one::Task1, two::Task2};

mod five;
mod four;
mod one;
mod seven;
mod six;
mod three;
mod two;

pub trait TaskCompleter {
    fn get_name(&self) -> String;
    fn do_task_1(&self) -> String;
    fn do_task_2(&self) -> String;
}

const NUMBER_OF_RUNS: i32 = 10;

fn main() {
    let filtered_tasks: Vec<i32> = std::env::args()
        .filter_map(|arg| arg.parse::<i32>().ok())
        .collect();
    let tasks: Vec<&dyn TaskCompleter> =
        vec![&Task1, &Task2, &Task3, &Task4, &Task5, &Task6, &Task7];

    let mut col_widths = [4, 13, 19, 13, 19];

    let results: Vec<[String; 5]> = zip(0.., tasks)
        .filter(|(index, _)| filtered_tasks.is_empty() || filtered_tasks.contains(&(index + 1)))
        .map(|(_, task)| {
            let mut task_1_durations = vec![];
            let start: Instant = Instant::now();
            let task_1_result = task.do_task_1();
            task_1_durations.push(start.elapsed());
            for _ in 0..NUMBER_OF_RUNS - 1 {
                let start: Instant = Instant::now();
                assert_eq!(task_1_result, task.do_task_1());
                task_1_durations.push(start.elapsed());
            }

            let mut task_2_durations = vec![];
            let start: Instant = Instant::now();
            let task_2_result = task.do_task_2();
            task_2_durations.push(start.elapsed());
            for _ in 0..NUMBER_OF_RUNS - 1 {
                let start: Instant = Instant::now();
                assert_eq!(task_2_result, task.do_task_2());
                let duration = start.elapsed();
                task_2_durations.push(duration);
            }
            let result = [
                task.get_name(),
                task_1_result,
                format!(
                    "{:?}",
                    (task_1_durations.iter().sum::<Duration>() / task_1_durations.len() as u32)
                ),
                task_2_result,
                format!(
                    "{:?}",
                    (task_2_durations.iter().sum::<Duration>() / task_2_durations.len() as u32)
                ),
            ];
            for i in 0..5 {
                if col_widths[i] < result[i].len() {
                    col_widths[i] = result[i].len();
                }
            }
            result
        })
        .collect();

    for [c1, c2, c3, c4, c5] in once([
        "Task".to_owned(),
        "Part 1 Result".to_owned(),
        "Part 1 Average Time".to_owned(),
        "Part 2 Result".to_owned(),
        "Part 2 Average Time".to_owned(),
    ])
    .chain(results)
    {
        println!(
            "| {:<width0$} | {:<width1$} | {:<width2$} | {:<width3$} | {:<width4$} |",
            c1,
            c2,
            c3,
            c4,
            c5,
            width0 = col_widths[0],
            width1 = col_widths[1],
            width2 = col_widths[2],
            width3 = col_widths[3],
            width4 = col_widths[4],
        );
    }
}
