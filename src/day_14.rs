use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::Display,
    hash::{Hash, Hasher},
    iter::zip,
    ops::{Index, IndexMut},
};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::TaskCompleter;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Tile {
    Moveable,
    Solid,
    Empty,
}

impl Tile {
    fn new(x: char) -> Tile {
        match x {
            'O' => Tile::Moveable,
            '#' => Tile::Solid,
            '.' => Tile::Empty,
            _ => panic!("Invalid character {}", x),
        }
    }

    fn to_char(&self) -> &str {
        match self {
            Tile::Moveable => "O",
            Tile::Solid => "#",
            Tile::Empty => ".",
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Grid {
    g: Vec<Vec<Tile>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "\n".to_owned();
        for y in 0..self.height() {
            let mut l = "".to_owned();

            for x in 0..self.width() {
                l += self[(x, y)].to_char();
            }
            str += &l;
            str += "\n";
        }
        f.write_str(&str)
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.g[index.1][index.0]
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.g[index.1][index.0]
    }
}

impl Grid {
    fn width(&self) -> usize {
        self.g[0].len()
    }
    fn height(&self) -> usize {
        self.g.len()
    }

    fn new(input: &str) -> Self {
        let v = input
            .lines()
            .map(|x| x.chars().map(Tile::new).collect())
            .collect();
        Grid { g: v }
    }

    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                let mut number_of_movable = 0;

                for j in 0..self.width() {
                    for i in (0..self.height()).rev() {
                        // Count number of movable objects before a #
                        match self[(j, i)] {
                            Tile::Moveable => {
                                number_of_movable += 1;
                                self[(j, i)] = Tile::Empty;
                            }
                            Tile::Solid => {
                                for k in i + 1..i + 1 + number_of_movable {
                                    self[(j, k)] = Tile::Moveable;
                                }
                                number_of_movable = 0;
                            }
                            Tile::Empty => (),
                        }
                    }
                    for k in 0..number_of_movable {
                        self[(j, k)] = Tile::Moveable;
                    }
                    number_of_movable = 0;
                }
            }
            Direction::South => {
                for i in (0..self.height() - 1).rev() {
                    for j in 0..self.width() {
                        // If there is a moveable tile move it up as high as possible
                        if self[(j, i)] == Tile::Moveable {
                            let mut move_to = i;
                            while move_to < self.height() - 1
                                && self[(j, move_to + 1)] == Tile::Empty
                            {
                                move_to += 1;
                            }

                            if self[(j, move_to)] == Tile::Empty {
                                let tile = &mut self[(j, i)];
                                *tile = Tile::Empty;
                                let tile_to_move_to = &mut self[(j, move_to)];
                                *tile_to_move_to = Tile::Moveable;
                            }
                        }
                    }
                }
            }
            Direction::West => {
                let w = self.width();
                self.g
                    .par_iter_mut()
                    .map(|x| {
                        for i in 1..w {
                            // If there is a moveable tile move it up as high as possible
                            if x[i] == Tile::Moveable {
                                let mut move_to = i;
                                while move_to > 0 && x[move_to - 1] == Tile::Empty {
                                    move_to -= 1;
                                }

                                if x[move_to] == Tile::Empty {
                                    let tile = &mut x[i];
                                    *tile = Tile::Empty;
                                    let tile_to_move_to = &mut x[move_to];
                                    *tile_to_move_to = Tile::Moveable;
                                }
                            }
                        }
                        0
                    })
                    .sum::<usize>();
            }
            Direction::East => {
                let w = self.width();
                self.g
                    .par_iter_mut()
                    .map(|x| {
                        for i in (0..w - 1).rev() {
                            // If there is a moveable tile move it up as high as possible
                            if x[i] == Tile::Moveable {
                                let mut move_to = i;
                                while move_to < w - 1 && x[move_to + 1] == Tile::Empty {
                                    move_to += 1;
                                }

                                if x[move_to] == Tile::Empty {
                                    let tile = &mut x[i];
                                    *tile = Tile::Empty;
                                    let tile_to_move_to = &mut x[move_to];
                                    *tile_to_move_to = Tile::Moveable;
                                }
                            }
                        }
                        0
                    })
                    .sum::<usize>();
            }
        }
    }

    fn get_load_value(&self) -> usize {
        zip(0.., self.g.iter())
            .map(|(height, v)| {
                v.iter().filter(|x| x == &&Tile::Moveable).count() * (self.height() - height)
            })
            .sum::<usize>()
    }
}
pub struct Task14;

impl TaskCompleter for Task14 {
    fn get_name(&self) -> String {
        "14".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_14/input");
        let mut grid = Grid::new(contents);
        grid.move_direction(Direction::North);
        grid.get_load_value().to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_14/input");
        let mut grid = Grid::new(contents);
        const TOTAL_ITERS: u64 = 1000000000;
        let mut seen_before = HashMap::<u64, u64>::new();
        let mut start_of_loop = 0;
        let mut length_of_loop = 0;
        for i in 0..TOTAL_ITERS {
            let mut s = DefaultHasher::new();
            grid.hash(&mut s);
            let hash = s.finish();
            if let Some(index) = seen_before.get(&hash) {
                start_of_loop = *index;
                length_of_loop = i - index;
                break;
            } else {
                seen_before.insert(hash, i);
            }
            grid.move_direction(Direction::North);
            grid.move_direction(Direction::West);
            grid.move_direction(Direction::South);
            grid.move_direction(Direction::East);
        }
        let position = (TOTAL_ITERS - start_of_loop) % length_of_loop;
        for _ in 0..position {
            grid.move_direction(Direction::North);
            grid.move_direction(Direction::West);
            grid.move_direction(Direction::South);
            grid.move_direction(Direction::East);
        }
        grid.get_load_value().to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("109638".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
