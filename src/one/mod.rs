use std::{fs, path::Path};

pub fn run_task_1() {
    let file_path = Path::new("./input/one/input");
    dbg!(&file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let c: u32 = contents
        .split("\n")
        .map(|line| {
            let mut iter = line.chars().filter(|x| x.is_ascii_digit());
            iter.next()
                .and_then(|x| {
                    let x = x.to_digit(10).expect("Expected character to be a digit");
                    iter.last()
                        .and_then(|y| {
                            Some(x * 10 + y.to_digit(10).expect("Expected character to be a digit"))
                        })
                        .or_else(|| Some(x * 10 + x))
                })
                .unwrap_or(0)
        })
        .sum();
    dbg!(c);
}

pub fn run_task_2() {
    let file_path = Path::new("./input/one/input");
    dbg!(&file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let c: i32 = contents
        .to_lowercase()
        .split("\n")
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
    dbg!(c);
}
