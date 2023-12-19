#![allow(dead_code)]

use std::{
    any::type_name,
    iter::{once, zip},
    time::{Duration, Instant},
};

use day_03::Task3;
use day_04::Task4;
use day_05::Task5;
use day_06::Task6;
use day_07::Task7;
use day_08::Task8;
use day_09::Task9;
use day_10::Task10;
use day_11::Task11;
use day_12::Task12;
use day_13::Task13;
use day_14::Task14;
use day_15::Task15;
use day_16::Task16;
use day_17::Task17;
use day_18::Task18;
use day_19::Task19;

use crate::{day_01::Task1, day_02::Task2};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod grid;

pub trait TaskCompleter {
    fn get_name_auto(&self) -> String {
        type_name::<Self>().split("::").last().unwrap()[4..].to_string()
    }
    fn do_task_1(&self) -> String;
    fn do_task_2(&self) -> String;
    fn task_1_result(&self) -> Option<String>;
    fn task_2_result(&self) -> Option<String>;
}

const NUMBER_OF_RUNS: i32 = 10;

fn main() {
    let tasks: Vec<&dyn TaskCompleter> = vec![
        &Task1, &Task2, &Task3, &Task4, &Task5, &Task6, &Task7, &Task8, &Task9, &Task10, &Task11,
        &Task12, &Task13, &Task14, &Task15, &Task16, &Task17, &Task18, &Task19,
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
                let task_1_result;
                let mut task_1_durations = vec![];
                let number_of_runs;
                if let Some(x) = task.task_1_result() {
                    task_1_result = x;
                    number_of_runs = NUMBER_OF_RUNS;
                } else {
                    let start: Instant = Instant::now();
                    task_1_result = task.do_task_1();
                    task_1_durations.push(start.elapsed());
                    number_of_runs = 0;
                }
                for _ in 0..number_of_runs {
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
                let task_2_result;
                let mut task_2_durations = vec![];
                let number_of_runs;
                if let Some(x) = task.task_2_result() {
                    task_2_result = x;
                    number_of_runs = NUMBER_OF_RUNS;
                } else {
                    let start: Instant = Instant::now();
                    task_2_result = task.do_task_2();
                    task_2_durations.push(start.elapsed());
                    number_of_runs = 0;
                }

                for _ in 0..number_of_runs {
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
                task.get_name_auto(),
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
