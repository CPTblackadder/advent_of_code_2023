use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Index, IndexMut},
    thread,
};

use crate::TaskCompleter;

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

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
struct Grid<T> {
    g: Vec<Vec<T>>,
}

impl Grid<char> {
    fn from_string(input: &str) -> Self {
        let v = input.lines().rev().map(|x| x.chars().collect()).collect();
        Grid { g: v }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "\n".to_owned();
        for y in 0..self.height() {
            let mut l = "".to_owned();

            for x in 0..self.width() {
                l += &self[Coord(x as i64, y as i64)].to_string();
            }
            str += &l;
            str += "\n";
        }
        f.write_str(&str)
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.g[index.1 as usize][index.0 as usize]
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.g[index.1 as usize][index.0 as usize]
    }
}

impl<T: Default + Clone> Grid<T> {
    fn default_with_size(width: usize, height: usize) -> Self {
        let row_def = vec![T::default(); width];
        let g = vec![row_def; height];
        Self { g }
    }
}

impl<T> Grid<T> {
    fn width(&self) -> usize {
        self.g[0].len()
    }
    fn height(&self) -> usize {
        self.g.len()
    }

    fn in_bounds(&self, d: Coord) -> bool {
        d.0 >= 0 && d.0 < self.width() as i64 && d.1 >= 0 && d.1 < self.height() as i64
    }
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Coord(i64, i64);

impl Coord {
    fn translate_no_bounds(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Left => Self(self.0 + 1, self.1),
            Direction::Right => Self(self.0 - 1, self.1),
        }
    }

    fn translate<T>(&self, dir: Direction, grid: &Grid<T>) -> Option<Self> {
        let d = match dir {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        };
        if grid.in_bounds(d) {
            Some(d)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn follow_light_ray(
    chars: &Grid<char>,
    rays: &mut Grid<LightRay>,
    init_from: Coord,
    init_dir: Direction,
) {
    let mut to_check = vec![(init_from, init_dir)];
    while let Some((from, direction)) = to_check.pop() {
        if chars.in_bounds(from) && rays[from].set(direction) {
            return;
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
}

pub struct Task16;

impl TaskCompleter for Task16 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_16/example");
        let chars = Grid::from_string(contents);
        let mut rays = Grid::default_with_size(chars.width(), chars.height());

        follow_light_ray(
            &chars,
            &mut rays,
            Coord(-1, chars.height() as i64 - 1),
            Direction::Right,
        );
        rays.g
            .iter()
            .flatten()
            .filter(|x| x.is_energized())
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_16/input");
        let chars = Grid::from_string(contents);

        (0..chars.width() as i64)
            .map(|x| (Coord(x, -1), Direction::Up))
            .chain(
                (0..chars.width() as i64)
                    .map(|x| (Coord(x, chars.height() as i64), Direction::Down)),
            )
            .chain((0..chars.height() as i64).map(|y| (Coord(-1, y), Direction::Right)))
            .chain(
                (0..chars.height() as i64)
                    .map(|y| (Coord(chars.width() as i64, y), Direction::Left)),
            )
            .map(|(c, d)| {
                let mut rays = Grid::default_with_size(chars.width(), chars.height());
                follow_light_ray(&chars, &mut rays, c, d);
                rays.g.iter().flatten().filter(|x| x.is_energized()).count()
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("7979".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
