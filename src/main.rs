use std::time::{Duration, Instant};

mod five;
mod four;
pub mod helpers;
mod one;
mod three;
mod two;

fn main() {
    let mut durations = vec![];
    let number_of_runs: i32 = 100;
    for _ in 0..10 {
        let start: Instant = Instant::now();
        five::run_task();
        let duration = start.elapsed();
        durations.push(duration);
    }
    println!(
        "Average duration over {} runs is: {:?}",
        number_of_runs,
        durations.iter().sum::<Duration>() / durations.len() as u32
    );
}
