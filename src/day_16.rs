use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    grid::{Coord, Direction, Grid},
    TaskCompleter,
};

#[derive(Default, Clone)]
struct LightRay {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl LightRay {
    // Returns true if already set
    fn set(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Up => {
                if self.up {
                    true
                } else {
                    self.up = true;
                    false
                }
            }
            Direction::Down => {
                if self.down {
                    true
                } else {
                    self.down = true;
                    false
                }
            }
            Direction::Left => {
                if self.left {
                    true
                } else {
                    self.left = true;
                    false
                }
            }
            Direction::Right => {
                if self.right {
                    true
                } else {
                    self.right = true;
                    false
                }
            }
        }
    }

    fn is_energized(&self) -> bool {
        self.down || self.up || self.right || self.left
    }
}

fn follow_light_ray(chars: &Grid<char>, init_from: Coord, init_dir: Direction) -> Grid<LightRay> {
    let mut rays: Grid<LightRay> = Grid::default_with_size(chars.width(), chars.height());
    let mut to_check = vec![(init_from, init_dir)];
    while let Some((from, direction)) = to_check.pop() {
        if chars.in_bounds(from) && rays[from].set(direction) {
            continue;
        }
        if let Some(new_coord) = from.translate(direction, chars) {
            match chars[new_coord] {
                '-' => match direction {
                    Direction::Up | Direction::Down => {
                        to_check.push((new_coord, Direction::Left));
                        to_check.push((new_coord, Direction::Right));
                    }
                    Direction::Left | Direction::Right => {
                        to_check.push((new_coord, direction));
                    }
                },
                '|' => match direction {
                    Direction::Left | Direction::Right => {
                        to_check.push((new_coord, Direction::Up));
                        to_check.push((new_coord, Direction::Down));
                    }
                    Direction::Up | Direction::Down => {
                        to_check.push((new_coord, direction));
                    }
                },
                '/' => {
                    let new_dir = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    to_check.push((new_coord, new_dir));
                }
                '\\' => {
                    let new_dir = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    to_check.push((new_coord, new_dir));
                }
                '.' => to_check.push((new_coord, direction)),
                _ => panic!("Invalic character {}", chars[new_coord]),
            }
        }
    }
    rays
}

pub struct Task16;

impl TaskCompleter for Task16 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_16/input");
        let chars = Grid::from_string(contents, true);
        let rays = follow_light_ray(
            &chars,
            Coord::new(-1, chars.height() as i64 - 1),
            Direction::Right,
        );
        rays.grid()
            .iter()
            .flatten()
            .filter(|x| x.is_energized())
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_16/input");
        let chars = Grid::from_string(contents, true);

        (0..chars.width() as i64)
            .into_par_iter()
            .map(|x| (Coord::new(x, -1), Direction::Up))
            .chain(
                (0..chars.width() as i64)
                    .into_par_iter()
                    .map(|x| (Coord::new(x, chars.height() as i64), Direction::Down)),
            )
            .chain(
                (0..chars.height() as i64)
                    .into_par_iter()
                    .map(|y| (Coord::new(-1, y), Direction::Right)),
            )
            .chain(
                (0..chars.height() as i64)
                    .into_par_iter()
                    .map(|y| (Coord::new(chars.width() as i64, y), Direction::Left)),
            )
            .map(|(c, d)| {
                let rays = follow_light_ray(&chars, c, d);
                rays.grid()
                    .iter()
                    .flatten()
                    .filter(|x| x.is_energized())
                    .count()
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("7979".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("8437".to_owned())
    }
}
