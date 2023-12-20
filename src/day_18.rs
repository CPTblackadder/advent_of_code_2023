use std::{cmp::Ordering, ops::Range};

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

fn solve_by_vertical_edges(mut vertical_edges: Vec<(i64, Range<i64>)>) -> String {
    vertical_edges.sort_by(|(_, y1), (_, y2)| {
        let start = y1.start.cmp(&y2.start);
        if start == Ordering::Equal {
            y1.end.cmp(&y2.end)
        } else {
            start
        }
    });
    vertical_edges.reverse();
    dbg!(&vertical_edges);

    let e1 = vertical_edges.pop().unwrap();
    let e2 = vertical_edges.pop().unwrap();
    let mut current_edges = vec![e1, e2];
    let mut area = 0;
    let mut y = current_edges[0].1.start;
    assert_eq!(y, current_edges[1].1.start);
    while !current_edges.is_empty() {
        dbg!(y);
        dbg!(area);
        dbg!(&current_edges);
        // Either remove current edges becaues their ended, or add a new edge because it's started
        current_edges.sort_by(|(_, y1), (_, y2)| y1.end.cmp(&y2.end));
        let first_edge_end = current_edges[0].1.end;
        let vertical_edges_next_start = vertical_edges[0].1.start;
        // Add area of all edges so far
        let new_y = vertical_edges_next_start;
        // Pair edges in current_edges by x value
        current_edges.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));
        for i in 0..current_edges.len() / 2 {
            // Should always be pairs of edges
            area += (current_edges[i * 2 + 1].0 - current_edges[i * 2].0 + 1) * (new_y - y);
        }
        y = new_y;

        if vertical_edges.len() > 0 && first_edge_end > vertical_edges_next_start {
            // Add all new edges that start at this value
            while vertical_edges.len() > 0
                && vertical_edges[vertical_edges.len() - 1].1.start == vertical_edges_next_start
            {
                current_edges.push(vertical_edges.pop().unwrap());
            }
        } else {
            while current_edges.len() > 0 && current_edges[0].1.end == first_edge_end {
                current_edges.remove(0);
            }
        }
    }
    area.to_string()
}

impl TaskCompleter for Task18 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_18/example");
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
        let mut vertical_edges = vec![];

        for line in contents.lines().rev() {
            let mut split = line.split(" ");
            let dir = get_dir(split.next().unwrap());
            let distance = split.next().unwrap().parse::<i64>().unwrap();
            match dir {
                Direction::Up => {
                    for i in pos.1..pos.1 + distance + 1 {
                        grid[Coord::new(pos.0, i)] = '#';
                    }
                    vertical_edges.push((pos.0, pos.1..pos.1 + distance));
                    pos.1 += distance;
                }
                Direction::Down => {
                    for i in pos.1 - distance..pos.1 {
                        grid[Coord::new(pos.0, i)] = '#';
                    }
                    vertical_edges.push((pos.0, pos.1 - distance..pos.1));
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

        let _ = std::io::Write::write_all(
            &mut std::fs::File::create("output2.txt").unwrap(),
            format!("{}", grid).as_bytes(),
        );

        grid.do_flood_fill(Coord::new(0, 0), 'O', '.', false);

        let res1 = grid
            .into_iter()
            .filter(|(_, c)| c != &&'O')
            .count()
            .to_string();

        let res2 = solve_by_vertical_edges(vertical_edges);
        assert_eq!(&res1, &res2);
        res1
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_18/input");
        let mut max_extent = (0, 0);
        let mut min_extent = (0, 0);
        let mut current_pos = (0, 0);
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
            let distance = i64::from_str_radix(&final_str[2..7], 16).unwrap();
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
        let mut vertical_edges = vec![];
        let mut pos = starting_pos;

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
            let distance = i64::from_str_radix(&final_str[2..7], 16).unwrap();
            match dir {
                Direction::Up => {
                    vertical_edges.push((pos.0, pos.1..pos.1 + distance));
                    pos.1 += distance;
                }
                Direction::Down => {
                    vertical_edges.push((pos.0, pos.1 - distance..pos.1));
                    pos.1 -= distance;
                }
                Direction::Left => {
                    pos.0 += distance;
                }
                Direction::Right => {
                    pos.0 -= distance;
                }
            }
        }
        solve_by_vertical_edges(vertical_edges)
    }

    fn task_1_result(&self) -> Option<String> {
        Some("35244".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
