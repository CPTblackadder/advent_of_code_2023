use std::collections::HashSet;

use crate::TaskCompleter;

fn parse_numbers(numbers: &str) -> HashSet<u32> {
    let mut set = HashSet::new();
    for num in numbers.split(" ") {
        if !num.is_empty() {
            set.insert(num.parse::<u32>().unwrap());
        }
    }
    set
}

pub struct Task4;

impl TaskCompleter for Task4 {
    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/four/input");
        let mut task_1_sum = 0;
        for line in contents.lines() {
            let _card_number = &line[5..8].trim().parse::<u32>().unwrap() - 1;
            let winning_numbers = parse_numbers(&line[9..40]);
            let my_numbers = parse_numbers(&line[41..]);
            let winning_matches = winning_numbers.intersection(&my_numbers).count() as u32;
            if winning_matches != 0 {
                task_1_sum += (2 as u32).pow(winning_matches - 1);
            }
        }
        task_1_sum.to_string()
    }

    fn do_task_2(&self) -> String {
        let contents = include_str!("../input/four/input");
        let mut card_copies = [1 as u32; 198];
        for line in contents.lines() {
            let card_number = &line[5..8].trim().parse::<u32>().unwrap() - 1;
            let winning_numbers = parse_numbers(&line[9..40]);
            let my_numbers = parse_numbers(&line[41..]);
            let winning_matches = winning_numbers.intersection(&my_numbers).count() as u32;
            for i in (card_number + 1)..(card_number + winning_matches + 1) {
                card_copies[i as usize] += card_copies[card_number as usize];
            }
        }
        card_copies.iter().sum::<u32>().to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("17782".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("8477787".to_owned())
    }
}
