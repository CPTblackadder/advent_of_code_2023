use std::{fs, path::Path};

use crate::TaskCompleter;

pub struct Task1;

impl TaskCompleter for Task1 {
    fn do_task_1(&self) -> String {
        let c: u32 = include_str!("../input/one/input")
            .lines()
            .map(|line| {
                let mut iter = line.chars().filter(|x| x.is_ascii_digit());
                iter.next()
                    .and_then(|x| {
                        let x = x.to_digit(10).expect("Expected character to be a digit");
                        iter.last()
                            .and_then(|y| {
                                Some(
                                    x * 10
                                        + y.to_digit(10).expect("Expected character to be a digit"),
                                )
                            })
                            .or_else(|| Some(x * 10 + x))
                    })
                    .unwrap_or(0)
            })
            .sum();
        c.to_string()
    }

    fn do_task_2(&self) -> String {
        let c: i32 = include_str!("../input/one/input")
            .to_lowercase()
            .lines()
            .map(|line| {
                let mut l: Vec<char> = line.chars().collect();
                let mut v = Vec::new();

                while l.len() != 0 {
                    let (number, remove) = match &l[0..] {
                        ['0', ..] => (Some(0), 1),
                        ['1', ..] => (Some(1), 1),
                        ['2', ..] => (Some(2), 1),
                        ['3', ..] => (Some(3), 1),
                        ['4', ..] => (Some(4), 1),
                        ['5', ..] => (Some(5), 1),
                        ['6', ..] => (Some(6), 1),
                        ['7', ..] => (Some(7), 1),
                        ['8', ..] => (Some(8), 1),
                        ['9', ..] => (Some(9), 1),
                        ['z', 'e', 'r', 'o', ..] => (Some(0), 4),
                        ['o', 'n', 'e', ..] => (Some(1), 3),
                        ['t', 'w', 'o', ..] => (Some(2), 3),
                        ['t', 'h', 'r', 'e', 'e', ..] => (Some(3), 5),
                        ['f', 'o', 'u', 'r', ..] => (Some(4), 4),
                        ['f', 'i', 'v', 'e', ..] => (Some(5), 4),
                        ['s', 'i', 'x', ..] => (Some(6), 3),
                        ['s', 'e', 'v', 'e', 'n', ..] => (Some(7), 5),
                        ['e', 'i', 'g', 'h', 't', ..] => (Some(8), 5),
                        ['n', 'i', 'n', 'e', ..] => (Some(9), 4),
                        _ => (None, 1),
                    };
                    if let Some(n) = number {
                        v.push(n);
                    }
                    for _ in [0..remove] {
                        l.remove(0);
                    }
                }
                let mut iter = v.iter();
                iter.next()
                    .and_then(|x| {
                        iter.last()
                            .and_then(|y| Some(x * 10 + y))
                            .or_else(|| Some(x * 10 + x))
                    })
                    .unwrap_or(0)
            })
            .sum();
        c.to_string()
    }

    fn get_name(&self) -> String {
        "1".to_owned()
    }
}
