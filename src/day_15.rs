use std::iter::zip;

use crate::TaskCompleter;

fn hash(input: &str) -> u64 {
    input.chars().fold(0, |v, c| ((v + (c as u64)) * 17) % 256)
}

fn handle_str<'a>(v: &mut [Vec<(&'a str, u64)>; 256], x: &'a str) {
    if x.contains("-") {
        let mut split = x.split("-");
        let chars = split.next().unwrap();
        let box_number = hash(chars);
        if let Some(index) = v[box_number as usize].iter().position(|(x, _)| x == &chars) {
            v[box_number as usize].remove(index);
        }
    } else {
        let mut split = x.split("=");
        let chars = split.next().unwrap();
        let focul_length = split.next().unwrap().parse::<u64>().unwrap();
        let box_number = hash(chars);
        if let Some(index) = v[box_number as usize].iter().position(|(x, _)| x == &chars) {
            v[box_number as usize][index] = (chars, focul_length);
        } else {
            v[box_number as usize].push((chars, focul_length));
        }
    }
}

pub struct Task15;

impl TaskCompleter for Task15 {
    fn get_name(&self) -> String {
        "15".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_15/input");
        contents.split(",").map(hash).sum::<u64>().to_string()
    }

    fn do_task_2(&self) -> String {
        const V: Vec<(&str, u64)> = Vec::new();
        let contents: &str = include_str!("../input/day_15/input");

        zip(
            1..,
            contents.split(",").fold([V; 256], |mut v, x| {
                handle_str(&mut v, x);
                v
            }),
        )
        .map(|(i, b)| {
            zip(1.., b.iter())
                .map(|(j, (_, focul_length))| i * j * focul_length)
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("517551".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("286097".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{day_08::str_to_index, day_15::hash};

    #[test]
    fn hash_test() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("qp"), 1);
        assert_eq!(hash("pc"), 3);
    }
}
