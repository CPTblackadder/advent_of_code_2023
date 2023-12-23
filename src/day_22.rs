use std::{iter::zip, ops::Range};

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::{
    grid::{Coord, Grid},
    TaskCompleter,
};

pub struct Task22;

#[derive(Debug, PartialEq, Clone)]
enum Brick {
    Cube(i64, i64, i64),
    X(Range<i64>, i64, i64),
    Y(i64, Range<i64>, i64),
    Z(i64, i64, Range<i64>),
}

#[derive(Debug)]
struct BrickRestsOn {
    rests_on: Vec<usize>,
    is_rested_on_by: Vec<usize>,
}

impl Brick {
    fn new(str: &str) -> Self {
        let mut parts = str.split(&['~', ',']);
        let start_x = parts.next().unwrap().parse::<i64>().unwrap();
        let start_y = parts.next().unwrap().parse::<i64>().unwrap();
        let start_z = parts.next().unwrap().parse::<i64>().unwrap();
        let end_x = parts.next().unwrap().parse::<i64>().unwrap();
        let end_y = parts.next().unwrap().parse::<i64>().unwrap();
        let end_z = parts.next().unwrap().parse::<i64>().unwrap();

        // Ranges are one extra as we want inclusive ranges
        if start_x < end_x {
            assert_eq!(start_y, end_y);
            assert_eq!(start_z, end_z);
            Brick::X(start_x..end_x + 1, start_y, start_z)
        } else if start_y < end_y {
            assert_eq!(start_x, end_x);
            assert_eq!(start_z, end_z);
            Brick::Y(start_x, start_y..end_y + 1, start_z)
        } else if start_z < end_z {
            assert_eq!(start_x, end_x);
            assert_eq!(start_y, end_y);
            Brick::Z(start_x, start_y, start_z..end_z + 1)
        } else {
            assert_eq!(start_x, end_x);
            assert_eq!(start_y, end_y);
            assert_eq!(start_z, end_z);
            Brick::Cube(start_x, start_y, start_z)
        }
    }

    fn max_dimensions(&self) -> ((i64, i64), (i64, i64), (i64, i64)) {
        match self {
            Brick::Cube(x, y, z) => ((*x, *x), (*y, *y), (*z, *z)),
            Brick::X(xs, y, z) => ((xs.start, xs.end), (*y, *y), (*z, *z)),
            Brick::Y(x, ys, z) => ((*x, *x), (ys.start, ys.end), (*z, *z)),
            Brick::Z(x, y, zs) => ((*x, *x), (*y, *y), (zs.start, zs.end)),
        }
    }

    fn fall(&mut self, g: &mut Grid<i64>) -> bool {
        let fell;
        match self {
            Brick::Cube(x, y, z) => {
                let c = Coord::new(*x, *y);
                fell = *z > g[c] + 1;
                *z = g[c] + 1;
                g[c] = *z;
            }
            Brick::X(xs, y, z) => {
                let new_z = xs.clone().map(|x| g[Coord::new(x, *y)] + 1).max().unwrap();
                fell = *z > new_z;
                *z = new_z;
                for x in xs.clone() {
                    g[Coord::new(x, *y)] = new_z;
                }
            }
            Brick::Y(x, ys, z) => {
                let new_z = ys.clone().map(|y| g[Coord::new(*x, y)] + 1).max().unwrap();
                fell = *z > new_z;
                *z = new_z;
                for y in ys.clone() {
                    g[Coord::new(*x, y)] = new_z;
                }
            }
            Brick::Z(x, y, zs) => {
                let c = Coord::new(*x, *y);
                let z_length = zs.end - zs.start;
                let new_z_start = g[c] + 1;
                fell = new_z_start < zs.start;
                *zs = new_z_start..new_z_start + z_length;
                // Exclusive range
                g[c] = zs.end - 1;
            }
        }
        fell
    }

    fn rests_on(&self, y: &Brick) -> bool {
        let coords_self = match self {
            Brick::Cube(x, y, z) => vec![(*x, *y, z - 1)],
            Brick::X(xs, y, z) => xs.clone().map(|x| (x, *y, z - 1)).collect(),
            Brick::Y(x, ys, z) => ys.clone().map(|y| (*x, y, z - 1)).collect(),
            Brick::Z(x, y, zs) => vec![(*x, *y, zs.start - 1)],
        };

        coords_self.iter().any(|x| y.contains(x))
    }

    fn can_fall_further(&self, bricks: &Vec<Brick>) -> bool {
        let bottom_z = self.max_dimensions().2 .0;
        let is_on_floor = bottom_z == 1;
        let is_resting_on_something = bricks.iter().any(|x| self.rests_on(x));
        !is_on_floor && !is_resting_on_something
    }

    fn contains(&self, (t_x, t_y, t_z): &(i64, i64, i64)) -> bool {
        match self {
            Brick::Cube(s_x, s_y, s_z) => t_x == s_x && t_y == s_y && t_z == s_z,
            Brick::X(xs, s_y, s_z) => t_y == s_y && t_z == s_z && xs.clone().any(|x| x == *t_x),
            Brick::Y(s_x, ys, s_z) => t_x == s_x && t_z == s_z && ys.clone().any(|y| y == *t_y),
            Brick::Z(s_x, s_y, zs) => t_x == s_x && t_y == s_y && zs.clone().any(|z| z == *t_z),
        }
    }
}

fn get_largest_dimensions(
    first: ((i64, i64), (i64, i64), (i64, i64)),
    second: ((i64, i64), (i64, i64), (i64, i64)),
) -> ((i64, i64), (i64, i64), (i64, i64)) {
    (
        (first.0 .0.min(second.0 .0), first.0 .1.max(second.0 .1)),
        (first.1 .0.min(second.1 .0), first.1 .1.max(second.1 .1)),
        (first.2 .0.min(second.2 .0), first.2 .1.max(second.2 .1)),
    )
}

fn destroy_and_fall(index: usize, mut bricks: Vec<Brick>, max_dimensions: (i64, i64)) -> i64 {
    bricks.swap_remove(index);
    fall_bricks(&mut bricks, max_dimensions)
}

fn fall_bricks(bricks: &mut Vec<Brick>, max_dimensions: (i64, i64)) -> i64 {
    let mut sorted_indexes = (0..bricks.len()).into_iter().collect::<Vec<usize>>();
    sorted_indexes.sort_by(|x, y| {
        bricks[*x]
            .max_dimensions()
            .2
             .0
            .cmp(&bricks[*y].max_dimensions().2 .0)
    });
    let (_, fell_count) = (0..sorted_indexes.len()).into_iter().fold(
        (
            Grid::default_with_size(max_dimensions.0 as usize, max_dimensions.1 as usize),
            0,
        ),
        |(mut g, fell_count), i| {
            let i = sorted_indexes[i];
            let fell = bricks[i].fall(&mut g);
            let fell_count = if fell { fell_count + 1 } else { fell_count };
            (g, fell_count)
        },
    );
    // if let Some(brick_to_fall_fruther) = bricks.iter().find(|x| x.can_fall_further(&bricks)) {
    //     panic!("Brick {:?}, can fall down further", brick_to_fall_fruther);
    // }
    fell_count
}

impl TaskCompleter for Task22 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_22/input");
        let mut bricks = contents.lines().map(Brick::new).collect::<Vec<Brick>>();
        let max_dimensions = bricks.iter().fold(((0, 0), (0, 0), (0, 0)), |r, b| {
            get_largest_dimensions(r, b.max_dimensions())
        });
        fall_bricks(&mut bricks, (max_dimensions.0 .1, max_dimensions.1 .1));
        let rests_on = bricks
            .par_iter()
            .map(|b| {
                let mut rests_on = vec![];
                let mut is_rested_on_by = vec![];
                for (i, b2) in zip(0.., bricks.iter()) {
                    if b.rests_on(b2) {
                        rests_on.push(i);
                    } else if b2.rests_on(b) {
                        is_rested_on_by.push(i);
                    }
                }
                BrickRestsOn {
                    rests_on,
                    is_rested_on_by,
                }
            })
            .collect::<Vec<BrickRestsOn>>();

        rests_on
            .iter()
            .filter(|b| {
                b.is_rested_on_by
                    .iter()
                    .filter(|y| rests_on[**y].rests_on.len() <= 1)
                    .count()
                    == 0
            })
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_22/input");
        let mut bricks = contents.lines().map(Brick::new).collect::<Vec<Brick>>();
        let max_dimensions = bricks.iter().fold(((0, 0), (0, 0), (0, 0)), |r, b| {
            get_largest_dimensions(r, b.max_dimensions())
        });
        let max_dimensions = (max_dimensions.0 .1, max_dimensions.1 .1);
        fall_bricks(&mut bricks, max_dimensions);

        (0..bricks.len())
            .into_par_iter()
            .map(|i| destroy_and_fall(i, bricks.clone(), max_dimensions))
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("499".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("95059".to_owned())
    }
}
