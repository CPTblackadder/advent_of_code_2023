use std::iter::zip;

use crate::TaskCompleter;

pub struct Task11;

impl TaskCompleter for Task11 {
    fn get_name(&self) -> String {
        "11".to_owned()
    }

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
        let mut sum = 0;

        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let (x1, y1) = galaxies[i];
                let (x2, y2) = galaxies[j];
                sum += x1.abs_diff(x2) + y1.abs_diff(y2)
            }
        }

        sum.to_string()
    }

    fn do_task_2(&self) -> String {
        "todo!()".to_owned()
    }
}
