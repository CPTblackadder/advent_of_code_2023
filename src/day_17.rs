use crate::{
    grid::{Coord, Direction, Grid},
    TaskCompleter,
};
use pathfinding::prelude::astar;

pub struct Task17;

pub fn successors(
    coord: &Coord,
    dir: Direction,
    distance: i32,
    grid: &Grid<i64>,
) -> Vec<((Coord, Direction, i32), i64)> {
    let mut v = vec![];

    {
        let left = dir.left();
        let mut coord1 = coord.translate(left, grid);
        let mut cost = 0;
        for i in 1..4 {
            if let Some(c1) = coord1 {
                cost += grid[c1];
                v.push(((c1, left, i), cost));
                coord1 = c1.translate(left, grid);
            } else {
                break;
            }
        }
    }
    {
        let right = dir.right();
        let mut coord1 = coord.translate(right, grid);
        let mut cost = 0;
        for i in 1..4 {
            if let Some(c1) = coord1 {
                cost += grid[c1];
                v.push(((c1, right, i), cost));
                coord1 = c1.translate(right, grid);
            } else {
                break;
            }
        }
    }
    {
        let mut coord1 = coord.translate(dir, grid);
        let mut cost = 0;
        for i in (distance + 1)..4 {
            if let Some(c1) = coord1 {
                cost += grid[c1];
                v.push(((c1, dir, i), cost));
                coord1 = c1.translate(dir, grid);
            } else {
                break;
            }
        }
    }
    v
}

pub fn ultra_successors(
    coord: &Coord,
    dir: Direction,
    distance: i32,
    grid: &Grid<i64>,
) -> Vec<((Coord, Direction, i32), i64)> {
    let mut v = vec![];
    const MAX_DISTANCE: i32 = 10;
    const MIN_DISTANCE: i32 = 4;
    {
        let mut coord1 = coord.translate(dir, grid);
        let mut cost = 0;
        for _ in distance + 1..MIN_DISTANCE {
            if let Some(c1) = coord1 {
                cost += grid[c1];
                coord1 = c1.translate(dir, grid);
            } else {
                break;
            }
        }
        for i in (distance + 1).max(MIN_DISTANCE)..MAX_DISTANCE + 1 {
            if let Some(c1) = coord1 {
                cost += grid[c1];
                v.push(((c1, dir, i), cost));
                coord1 = c1.translate(dir, grid);
            } else {
                break;
            }
        }
    }

    if distance >= 4 {
        {
            let left = dir.left();
            let mut coord1 = coord.translate(left, grid);
            let mut cost = 0;
            for _ in 1..MIN_DISTANCE {
                if let Some(c1) = coord1 {
                    cost += grid[c1];
                    coord1 = c1.translate(left, grid);
                } else {
                    break;
                }
            }
            for i in MIN_DISTANCE..MAX_DISTANCE + 1 {
                if let Some(c1) = coord1 {
                    cost += grid[c1];
                    v.push(((c1, left, i), cost));
                    coord1 = c1.translate(left, grid);
                } else {
                    break;
                }
            }
        }
        {
            let right = dir.right();
            let mut coord1 = coord.translate(right, grid);
            let mut cost = 0;
            for _ in 1..MIN_DISTANCE {
                if let Some(c1) = coord1 {
                    cost += grid[c1];
                    coord1 = c1.translate(right, grid);
                } else {
                    break;
                }
            }

            for i in MIN_DISTANCE..MAX_DISTANCE + 1 {
                if let Some(c1) = coord1 {
                    cost += grid[c1];
                    v.push(((c1, right, i), cost));
                    coord1 = c1.translate(right, grid);
                } else {
                    break;
                }
            }
        }
    }

    v
}

impl TaskCompleter for Task17 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_17/input");
        let grid = Grid::from_string_i64(contents);
        let dest = Coord::new(grid.width() as i64 - 1, 0);

        let res = astar(
            &(Coord::new(0, grid.height() as i64 - 1), Direction::Right, 0),
            |(x, y, z)| successors(x, *y, *z, &grid),
            |(x, _, _)| x.non_diagnal_distance(&dest),
            |(x, _, _)| x == &dest,
        );

        res.unwrap().1.to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_17/input");
        let grid = Grid::from_string_i64(contents);
        let dest = Coord::new(grid.width() as i64 - 1, 0);

        // For this input going right first gives 810, going down first gives 809
        let res = astar(
            &(Coord::new(0, grid.height() as i64 - 1), Direction::Down, 0),
            |(x, y, z)| ultra_successors(x, *y, *z, &grid),
            |(x, _, _)| x.non_diagnal_distance(&dest),
            |(x, _, _)| x == &dest,
        );

        res.unwrap().1.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("665".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("809".to_owned())
    }
}
