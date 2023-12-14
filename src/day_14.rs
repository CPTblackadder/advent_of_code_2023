use std::{
    collections::HashMap,
    fmt::Display,
    iter::zip,
    ops::{Index, IndexMut},
};

use indicatif::ProgressBar;

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
                for i in 1..self.height() {
                    for j in 0..self.width() {
                        // If there is a moveable tile move it up as high as possible
                        if self[(j, i)] == Tile::Moveable {
                            let mut move_to = i;
                            while move_to > 0 && self[(j, move_to - 1)] == Tile::Empty {
                                move_to -= 1;
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
                for j in 1..self.width() {
                    for i in 0..self.height() {
                        // If there is a moveable tile move it up as high as possible
                        if self[(j, i)] == Tile::Moveable {
                            let mut move_to = j;
                            while move_to > 0 && self[(move_to - 1, i)] == Tile::Empty {
                                move_to -= 1;
                            }

                            if self[(j, move_to)] == Tile::Empty {
                                let tile = &mut self[(j, i)];
                                *tile = Tile::Empty;
                                let tile_to_move_to = &mut self[(move_to, i)];
                                *tile_to_move_to = Tile::Moveable;
                            }
                        }
                    }
                }
            }
            Direction::East => {
                for j in (0..self.width() - 1).rev() {
                    for i in 0..self.height() {
                        // If there is a moveable tile move it up as high as possible
                        if self[(j, i)] == Tile::Moveable {
                            let mut move_to = j;
                            while move_to < self.width() - 1
                                && self[(move_to + 1, i)] == Tile::Empty
                            {
                                move_to += 1;
                            }

                            if self[(j, move_to)] == Tile::Empty {
                                let tile = &mut self[(j, i)];
                                *tile = Tile::Empty;
                                let tile_to_move_to = &mut self[(move_to, i)];
                                *tile_to_move_to = Tile::Moveable;
                            }
                        }
                    }
                }
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
        let contents: &str = include_str!("../input/day_14/example");
        let mut grid = Grid::new(contents);
        let p = ProgressBar::new(1000);
        for i in 0..1000000000 {
            if i % 1000000 == 0 {
                p.inc(1);
            }
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
