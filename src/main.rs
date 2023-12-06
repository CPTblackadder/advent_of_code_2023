use std::time::{Duration, Instant};

mod five;
mod four;
pub mod helpers;
mod one;
mod six;
mod three;
mod two;

fn main() {
    let mut durations = vec![];
    let number_of_runs: i32 = 1;
    for _ in 0..number_of_runs {
        let start: Instant = Instant::now();
        six::run_task_2();
        let duration = start.elapsed();
        durations.push(duration);
    }
    println!(
        "Average duration over {} runs is: {:?}",
        number_of_runs,
        durations.iter().sum::<Duration>() / durations.len() as u32
    );
}
