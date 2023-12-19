use std::{fs::File, io::Write, os::windows};

use crate::{
    grid::{Coord, Direction, Grid},
    TaskCompleter,
};

pub struct Task18;

fn get_dir(input: &str) -> Direction {
    match input {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "R" => Direction::Right,
        "L" => Direction::Left,
        _ => panic!("Invalid character for Direction {}", input),
    }
}

fn set_limits<T, F>(arg1: (T, T), arg2: (T, T), f: F) -> (T, T)
where
    F: Fn(T, T) -> T,
    T: Ord,
{
    (f(arg1.0, arg2.0), f(arg1.1, arg2.1))
}

impl TaskCompleter for Task18 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_18/input");
        let mut max_extent = (0, 0);
        let mut min_extent = (0, 0);
        let mut current_pos = (0, 0);
        for line in contents.lines().rev() {
            let mut split = line.split(" ");
            let dir = get_dir(split.next().unwrap());
            let distance = split.next().unwrap().parse::<i64>().unwrap();
            let (pos_to_change, sign) = match dir {
                Direction::Up => (&mut current_pos.1, 1),
                Direction::Down => (&mut current_pos.1, -1),
                Direction::Left => (&mut current_pos.0, 1),
                Direction::Right => (&mut current_pos.0, -1),
            };
            *pos_to_change += sign * distance;
            max_extent = set_limits(current_pos, max_extent, i64::max);
            min_extent = set_limits(current_pos, min_extent, i64::min);
        }

        let max_extent = (max_extent.0 + 2, max_extent.1 + 2);
        let min_extent = (min_extent.0 - 1, min_extent.1 - 1);
        let starting_pos = (-min_extent.0, -min_extent.1);
        let dimensions = (max_extent.0 - min_extent.0, max_extent.1 - min_extent.1);
        let mut grid = Grid::init_with_size('.', dimensions.0 as usize, dimensions.1 as usize);
        let mut pos = starting_pos;

        for line in contents.lines().rev() {
            let mut split = line.split(" ");
            let dir = get_dir(split.next().unwrap());
            let distance = split.next().unwrap().parse::<i64>().unwrap();
            match dir {
                Direction::Up => {
                    for i in pos.1..pos.1 + distance + 1 {
                        grid[Coord::new(pos.0, i)] = '#';
                    }
                    pos.1 += distance;
                }
                Direction::Down => {
                    for i in pos.1 - distance..pos.1 {
                        grid[Coord::new(pos.0, i)] = '#';
                    }
                    pos.1 -= distance;
                }
                Direction::Left => {
                    for i in pos.0..pos.0 + distance + 1 {
                        grid[Coord::new(i, pos.1)] = '#';
                    }
                    pos.0 += distance;
                }
                Direction::Right => {
                    for i in pos.0 - distance..pos.0 {
                        grid[Coord::new(i, pos.1)] = '#';
                    }
                    pos.0 -= distance;
                }
            }
        }

        grid.do_flood_fill(Coord::new(0, 0), 'O', '.', false);

        grid.into_iter()
            .filter(|(_, c)| c != &&'O')
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_18/example");
        for line in contents.lines().rev() {
            let mut split = line.split(" ");
            split.next();
            split.next();
            let final_str = split.next().unwrap();
            let dir = match &final_str[7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("Invalid final character {} ", &final_str[7..8]),
            };
        }

        "todo".to_owned()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("35244".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
