use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Write},
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    grid::{Coord, Direction, Grid},
    TaskCompleter,
};

pub struct Task21;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug)]
enum Steps {
    #[default]
    NotEvaluated,
    Steps(i64),
}

impl Display for Steps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Steps::NotEvaluated => f.write_char('#'),
            Steps::Steps(x) => f.write_str(&x.to_string()),
        }
    }
}
impl Steps {
    fn is_even(&self, even: bool) -> bool {
        match self {
            Steps::NotEvaluated => false,
            Steps::Steps(x) => even == (x % 2 == 0),
        }
    }

    fn unwrap(&self) -> i64 {
        match self {
            Steps::NotEvaluated => panic!("Can't unwrap not evaluated steps"),
            Steps::Steps(x) => *x,
        }
    }
}

impl PartialEq<i64> for Steps {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Steps::NotEvaluated => false,
            Steps::Steps(x) => x == other,
        }
    }
}

impl PartialOrd<i64> for Steps {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        match self {
            Steps::NotEvaluated => Some(std::cmp::Ordering::Greater),
            Steps::Steps(x) => x.partial_cmp(other),
        }
    }
}

// Number of steps to go to a tile, if a tile is accessible then it can be got to every two after too
fn get_tiles_accessible_from(start: Coord, grid: &Grid<char>) -> Grid<Steps> {
    let mut steps = Grid::default_with_size(grid.width(), grid.height());
    let mut stack = VecDeque::new();
    stack.push_back((start, 0));
    while let Some((c, s)) = stack.pop_front() {
        if steps[c] <= s {
            continue;
        }
        steps[c] = Steps::Steps(s);
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(new_c) = c.translate(dir, grid) {
                if grid[new_c] != '#' {
                    stack.push_back((new_c, s + 1));
                }
            }
        }
    }
    steps
}

impl TaskCompleter for Task21 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_21/input");
        let g = Grid::from_string(contents, false);
        let starting_location = g.find_coord(|x| x == &'S').unwrap();
        let steps = get_tiles_accessible_from(starting_location, &g);
        steps
            .into_iter()
            .filter(|(_, s)| **s <= 64 && s.is_even(true))
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_21/example");
        let g = Grid::from_string(contents, false);
        let starting_location = g.find_coord(|x| x == &'S').unwrap();
        const TOTAL_STEPS: i64 = 50;
        const EVEN_STEPS: bool = TOTAL_STEPS % 2 == 0;

        let iter: Vec<(Coord, Grid<Steps>)> = (0..g.width())
            .into_par_iter()
            .map(|x| {
                let c1 = Coord::new(x as i64, 0);
                let c2 = Coord::new(x as i64, g.height() as i64 - 1);
                vec![
                    (c1, get_tiles_accessible_from(c1, &g)),
                    (c2, get_tiles_accessible_from(c2, &g)),
                ]
            })
            .flatten()
            .chain(
                (0..g.height())
                    .into_par_iter()
                    .map(|y| {
                        let c1 = Coord::new(0, y as i64);
                        let c2 = Coord::new(g.width() as i64 - 1, y as i64);
                        vec![
                            (c1, get_tiles_accessible_from(c1, &g)),
                            (c2, get_tiles_accessible_from(c2, &g)),
                        ]
                    })
                    .flatten(),
            )
            .collect();
        let mut steps_hash: HashMap<Coord, Grid<Steps>> = HashMap::from_iter(iter);
        steps_hash.insert(
            starting_location,
            get_tiles_accessible_from(starting_location, &g),
        );

        let starting_grid_steps = steps_hash
            .get(&starting_location)
            .unwrap()
            .into_iter()
            .filter(|x| x.1.is_even(EVEN_STEPS) && x.1.unwrap() < TOTAL_STEPS)
            .count() as i64;

        // For the straight bits



        // For four diagnols
        let diagonals = vec![
            (
                Coord::new(0, 0),
                Coord::new(g.width() as i64 - 1, g.height() as i64 - 1),
            ),
            (
                Coord::new(0, g.height() as i64 - 1),
                Coord::new(g.width() as i64 - 1, 0),
            ),
            (
                Coord::new(g.width() as i64 - 1, 0),
                Coord::new(0, g.height() as i64 - 1),
            ),
            (
                Coord::new(g.width() as i64 - 1, g.height() as i64 - 1),
                Coord::new(0, 0),
            ),
        ]
        .iter()
        .map(|(c, opp_c)| {
            let steps_to_cornet_from_start = steps_hash.get(&starting_location).unwrap()[*c];
            let steps_to_corner_from_corner = (g.height() + g.width() - 2) as i64;
            assert!(TOTAL_STEPS >= steps_to_cornet_from_start.unwrap());
            let (steps_to_edge, remainder) = num::integer::div_rem(
                TOTAL_STEPS - steps_to_cornet_from_start.unwrap(),
                steps_to_corner_from_corner,
            );
            let total_boxes_fully_filled = (steps_to_edge * (steps_to_edge + 1)) / 2;
            let even = steps_to_corner_from_corner % 2 == 0;
            let steps_in_full_box = steps_hash
                .get(&opp_c)
                .unwrap()
                .into_iter()
                .filter(|x| {
                    if even {
                        x.1.is_even(EVEN_STEPS)
                    } else {
                        x.1.is_even(!EVEN_STEPS)
                    }
                })
                .count() as i64;
            let steps_in_last_box = steps_hash
                .get(&opp_c)
                .unwrap()
                .into_iter()
                .filter(|x| {
                    if even {
                        x.1.is_even(EVEN_STEPS) && x.1.unwrap() <= remainder
                    } else {
                        x.1.is_even(!EVEN_STEPS) && x.1.unwrap() <= remainder
                    }
                })
                .count();
            (total_boxes_fully_filled * steps_in_full_box)
                + (steps_to_edge * steps_in_last_box as i64)
        })
        .sum::<i64>();

        (starting_grid_steps + diagonals).to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("3724".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
