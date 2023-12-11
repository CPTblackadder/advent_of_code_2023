use std::{
    iter::{once, zip},
    time::{Duration, Instant},
};

use day_08::Task8;
use day_09::Task9;
use day_10::Task10;
use day_11::Task11;
use five::Task5;
use four::Task4;
use seven::Task7;
use six::Task6;
use three::Task3;

use crate::{one::Task1, two::Task2};

mod day_08;
mod day_09;
mod day_10;
pub mod day_11;
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
    let tasks: Vec<&dyn TaskCompleter> = vec![
        &Task1, &Task2, &Task3, &Task4, &Task5, &Task6, &Task7, &Task8, &Task9, &Task10, &Task11,
    ];
    let mut bool_task_1 = false;
    let mut bool_task_2 = false;
    let mut omit_results = false;
    let filtered_tasks: Vec<i32> = std::env::args()
        .filter_map(|arg| {
            if arg == "--one" {
                bool_task_1 = true;
            }
            if arg == "--two" {
                bool_task_2 = true;
            }
            if arg == "--omit_results" {
                omit_results = true;
            }
            arg.parse::<i32>().ok()
        })
        .collect();

    let mut col_widths = [4, 13, 19, 13, 19];

    if !bool_task_1 && !bool_task_2 {
        bool_task_1 = true;
        bool_task_2 = true;
    }

    let results: Vec<[String; 5]> = zip(0.., tasks)
        .filter(|(index, _)| filtered_tasks.is_empty() || filtered_tasks.contains(&(index + 1)))
        .map(|(_, task)| {
            let (task_1_result, task_1_duration) = if bool_task_1 {
                let mut task_1_durations = vec![];
                let start: Instant = Instant::now();
                let task_1_result = task.do_task_1();
                task_1_durations.push(start.elapsed());
                for _ in 0..NUMBER_OF_RUNS - 1 {
                    let start: Instant = Instant::now();
                    assert_eq!(task_1_result, task.do_task_1());
                    task_1_durations.push(start.elapsed());
                }
                (
                    task_1_result,
                    format!(
                        "{:?}",
                        (task_1_durations.iter().sum::<Duration>() / task_1_durations.len() as u32)
                    ),
                )
            } else {
                ("".to_owned(), "".to_owned())
            };
            let (task_2_result, task_2_duration) = if bool_task_2 {
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
                (
                    task_2_result,
                    format!(
                        "{:?}",
                        (task_2_durations.iter().sum::<Duration>() / task_2_durations.len() as u32)
                    ),
                )
            } else {
                ("".to_owned(), "".to_owned())
            };
            let result = [
                task.get_name(),
                if !omit_results {
                    task_1_result
                } else {
                    "".to_owned()
                },
                task_1_duration,
                if !omit_results {
                    task_2_result
                } else {
                    "".to_owned()
                },
                task_2_duration,
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
