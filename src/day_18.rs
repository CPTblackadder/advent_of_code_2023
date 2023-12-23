use std::iter;

use crate::{
    grid::{Direction},
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

fn solve_by_vertices(vertices: &Vec<(i64, i64)>) -> i64 {
    let init_vertex = vertices[0];
    vertices[1..]
        .iter()
        .chain(iter::once(&init_vertex))
        .fold((0, init_vertex), |(area, i), j| {
            (area + (i.0 + j.0) * (j.1 - i.1), *j)
        })
        .0
        / 2
}

fn parse_line(line: &str) -> (Direction, i64) {
    let mut split = line.split(" ");
    let dir = get_dir(split.next().unwrap());
    let distance = split.next().unwrap().parse::<i64>().unwrap();
    (dir, distance)
}

fn parse_line_part2(line: &str) -> (Direction, i64) {
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
    let distance = i64::from_str_radix(&final_str[2..7], 16).unwrap();
    (dir, distance)
}

fn get_edges<F>(input: &str, parse_line_func: F) -> (Vec<(i64, i64)>, Vec<(i64, i64)>)
where
    F: Fn(&str) -> (Direction, i64),
{
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let (first_dir, first_distance) = parse_line_func(first_line);
    let mut previous_dir = first_dir;
    let mut right_vertices = vec![];
    let mut left_vertices = vec![];
    let mut top_left_corner_coord = match first_dir {
        Direction::Up => (0, -first_distance),
        Direction::Down => (0, first_distance),
        Direction::Left => (-first_distance, 0),
        Direction::Right => (first_distance, 0),
    };
    for line in lines.chain(iter::once(first_line)) {
        let (dir, distance) = parse_line_func(line);
        match dir {
            Direction::Up => {
                match previous_dir {
                    Direction::Right => {
                        left_vertices.push(top_left_corner_coord);
                        right_vertices
                            .push((top_left_corner_coord.0 + 1, top_left_corner_coord.1 + 1));
                    }
                    Direction::Left => {
                        left_vertices.push((top_left_corner_coord.0, top_left_corner_coord.1 + 1));
                        right_vertices.push((top_left_corner_coord.0 + 1, top_left_corner_coord.1));
                    }
                    _ => panic!(
                        "Invalid combo of directions: (dir: {:?}, previous_dir: {:?})",
                        dir, previous_dir
                    ),
                };
                top_left_corner_coord =
                    (top_left_corner_coord.0, top_left_corner_coord.1 - distance);
            }
            Direction::Down => {
                match previous_dir {
                    Direction::Right => {
                        left_vertices.push((top_left_corner_coord.0 + 1, top_left_corner_coord.1));
                        right_vertices.push((top_left_corner_coord.0, top_left_corner_coord.1 + 1));
                    }
                    Direction::Left => {
                        left_vertices
                            .push((top_left_corner_coord.0 + 1, top_left_corner_coord.1 + 1));
                        right_vertices.push(top_left_corner_coord);
                    }
                    _ => panic!(
                        "Invalid combo of directions: (dir: {:?}, previous_dir: {:?})",
                        dir, previous_dir
                    ),
                };
                top_left_corner_coord =
                    (top_left_corner_coord.0, top_left_corner_coord.1 + distance);
            }
            Direction::Left => {
                match previous_dir {
                    Direction::Up => {
                        left_vertices.push((top_left_corner_coord.0, top_left_corner_coord.1 + 1));
                        right_vertices.push((top_left_corner_coord.0 + 1, top_left_corner_coord.1));
                    }
                    Direction::Down => {
                        left_vertices
                            .push((top_left_corner_coord.0 + 1, top_left_corner_coord.1 + 1));
                        right_vertices.push(top_left_corner_coord);
                    }
                    _ => panic!(
                        "Invalid combo of directions: (dir: {:?}, previous_dir: {:?})",
                        dir, previous_dir
                    ),
                };
                top_left_corner_coord =
                    (top_left_corner_coord.0 - distance, top_left_corner_coord.1)
            }
            Direction::Right => {
                match previous_dir {
                    Direction::Up => {
                        left_vertices.push(top_left_corner_coord);
                        right_vertices
                            .push((top_left_corner_coord.0 + 1, top_left_corner_coord.1 + 1));
                    }
                    Direction::Down => {
                        left_vertices.push((top_left_corner_coord.0 + 1, top_left_corner_coord.1));
                        right_vertices.push((top_left_corner_coord.0, top_left_corner_coord.1 + 1));
                    }
                    _ => panic!(
                        "Invalid combo of directions: (dir: {:?}, previous_dir: {:?})",
                        dir, previous_dir
                    ),
                };
                top_left_corner_coord =
                    (top_left_corner_coord.0 + distance, top_left_corner_coord.1)
            }
        }
        previous_dir = dir;
    }
    (left_vertices, right_vertices)
}

impl TaskCompleter for Task18 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_18/input");
        let (left, right) = get_edges(contents, parse_line);
        solve_by_vertices(&left)
            .max(solve_by_vertices(&right))
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_18/input");
        let (left, right) = get_edges(contents, parse_line_part2);
        let res2 = solve_by_vertices(&left).max(solve_by_vertices(&right));
        res2.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("35244".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
