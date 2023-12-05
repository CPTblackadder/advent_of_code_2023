use std::collections::{HashMap, HashSet};

use crate::helpers::get_contents;

fn parse_numbers(numbers: &str) -> HashSet<u32> {
    let mut set = HashSet::new();
    for num in numbers.split(" ") {
        if !num.is_empty() {
            set.insert(num.parse::<u32>().unwrap());
        }
    }
    set
}

pub fn run_task() {
    let contents = get_contents("four".to_owned());
    let mut task_1_sum = 0;
    let mut card_copies = [1 as u32; 198];
    for line in contents.lines() {
        let card_number = &line[5..8].trim().parse::<u32>().unwrap() - 1;
        let winning_numbers = parse_numbers(&line[9..40]);
        let my_numbers = parse_numbers(&line[41..]);
        let winning_matches = winning_numbers.intersection(&my_numbers).count() as u32;
        if winning_matches != 0 {
            task_1_sum += (2 as u32).pow(winning_matches - 1);
        }
        for i in (card_number + 1)..(card_number + winning_matches + 1) {
            card_copies[i as usize] += card_copies[card_number as usize];
        }
    }
    println!("{}", task_1_sum);
    println!("{}", card_copies.iter().sum::<u32>());
}
