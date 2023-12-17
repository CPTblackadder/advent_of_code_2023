use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct Grid<T> {
    g: Vec<Vec<T>>,
}

impl Grid<i64> {
    pub fn from_string_i64(input: &str) -> Self {
        let v = input
            .lines()
            .rev()
            .map(|x| {
                x.chars()
                    .map(|y| y.to_string().parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        Grid { g: v }
    }
}

impl Grid<char> {
    pub fn from_string(input: &str) -> Self {
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
    pub fn default_with_size(width: usize, height: usize) -> Self {
        let row_def = vec![T::default(); width];
        let g = vec![row_def; height];
        Self { g }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.g[0].len()
    }
    pub fn height(&self) -> usize {
        self.g.len()
    }

    pub fn in_bounds(&self, d: Coord) -> bool {
        d.0 >= 0 && d.0 < self.width() as i64 && d.1 >= 0 && d.1 < self.height() as i64
    }

    pub fn grid(&self) -> &Vec<Vec<T>> {
        &self.g
    }
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Coord(i64, i64);

impl Coord {
    pub fn translate_no_bounds(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Left => Self(self.0 + 1, self.1),
            Direction::Right => Self(self.0 - 1, self.1),
        }
    }

    pub fn translate<T>(&self, dir: Direction, grid: &Grid<T>) -> Option<Self> {
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

    pub(crate) fn new(arg1: i64, arg2: i64) -> Coord {
        Coord(arg1, arg2)
    }

    pub(crate) fn non_diagnal_distance(&self, dest: &Coord) -> i64 {
        (self.0.abs_diff(dest.0) + self.1.abs_diff(dest.1)) as i64
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}
