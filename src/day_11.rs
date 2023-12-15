use std::iter::zip;

use crate::TaskCompleter;

pub struct Task11;

impl TaskCompleter for Task11 {
    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/day_11/input");
        let lines = contents.lines().collect::<Vec<&str>>();

        let mut empty_rows = vec![true; lines.len()];
        let mut empty_columns = vec![true; lines[0].len()];
        let mut galaxies = vec![];

        for (j, line) in zip(0.., lines) {
            for (i, char) in zip(0.., line.chars()) {
                match char {
                    '#' => {
                        empty_rows[j] = false;
                        empty_columns[i] = false;
                        galaxies.push((i, j));
                    }
                    '.' => (),
                    _ => panic!("Invalid character"),
                }
            }
        }

        let galaxies: Vec<(usize, usize)> = galaxies
            .iter()
            .map(|(x, y)| {
                (
                    x + empty_columns[0..*x].iter().filter(|x| **x).count(),
                    y + empty_rows[0..*y].iter().filter(|x| **x).count(),
                )
            })
            .collect();

        zip(0.., &galaxies)
            .map(|(i, (x1, y1))| {
                galaxies[i + 1..]
                    .iter()
                    .map(|(x2, y2)| x1.abs_diff(*x2) + y1.abs_diff(*y2))
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        const EXPANSION_FACTOR: usize = 999999;
        let contents = include_str!("../input/day_11/input");
        let lines = contents.lines().collect::<Vec<&str>>();

        let mut empty_rows = vec![true; lines.len()];
        let mut empty_columns = vec![true; lines[0].len()];
        let mut galaxies = vec![];

        for (j, line) in zip(0.., contents.lines()) {
            for (i, char) in zip(0.., line.chars()) {
                match char {
                    '#' => {
                        empty_rows[j] = false;
                        empty_columns[i] = false;
                        galaxies.push((i, j));
                    }
                    '.' => (),
                    _ => panic!("Invalid character"),
                }
            }
        }

        let galaxies: Vec<(usize, usize)> = galaxies
            .iter()
            .map(|(x, y)| {
                (
                    x + (EXPANSION_FACTOR * empty_columns[0..*x].iter().filter(|x| **x).count()),
                    y + (EXPANSION_FACTOR * empty_rows[0..*y].iter().filter(|x| **x).count()),
                )
            })
            .collect();

        zip(0.., &galaxies)
            .map(|(i, (x1, y1))| {
                galaxies[i + 1..]
                    .iter()
                    .map(|(x2, y2)| x1.abs_diff(*x2) + y1.abs_diff(*y2))
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("9177603".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("632003913611".to_owned())
    }
}
