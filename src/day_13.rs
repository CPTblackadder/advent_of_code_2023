use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::TaskCompleter;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn new(x: char) -> Tile {
        match x {
            '#' => Tile::Rock,
            '.' => Tile::Ash,
            _ => panic!("Invalid character {}", x),
        }
    }

    fn invert(&mut self) {
        *self = match self {
            Tile::Ash => Tile::Rock,
            Tile::Rock => Tile::Ash,
        }
    }

    fn to_char(&self) -> &str {
        match self {
            Tile::Ash => ".",
            Tile::Rock => "#",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReflectiveLine {
    Horizontal(usize),
    Vertical(usize),
}

impl ReflectiveLine {
    fn get_final_value(&self) -> usize {
        match self {
            ReflectiveLine::Horizontal(x) => x * 100,
            ReflectiveLine::Vertical(x) => *x,
        }
    }
}

#[derive(Clone, Debug)]
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

    fn new() -> Self {
        Grid { g: vec![] }
    }

    fn add_line(&mut self, char: Vec<char>) {
        self.g.push(
            char.into_iter()
                .map(|x| Tile::new(x))
                .collect::<Vec<Tile>>(),
        );
    }

    fn get_smudges(&self) -> Vec<Grid> {
        let mut v = vec![];
        // Get current reflective line
        // let horizontal_range;
        // let vertical_range;
        // match get_horizontal_reflection(self)
        //     .or_else(|| get_vertical_reflection(self))
        //     .unwrap()
        // {
        //     ReflectiveLine::Horizontal(x) => {
        //         horizontal_range = 0..self.width();
        //         let gap = (self.height() - x).min(x);
        //         vertical_range = (x - gap)..(x + gap);
        //     }
        //     ReflectiveLine::Vertical(y) => {
        //         vertical_range = 0..self.height();
        //         let gap = (self.width() - y).min(y);
        //         horizontal_range = (y - gap)..(y + gap);
        //     }
        // }
        // dbg!(&horizontal_range);
        // dbg!(&vertical_range);
        let horizontal_range = 0..self.width();
        let vertical_range = 0..self.height();

        for i in horizontal_range.to_owned() {
            for j in vertical_range.to_owned() {
                let mut new_g = self.clone();
                new_g[(i, j)].invert();
                v.push(new_g);
            }
        }
        v
    }
}

fn verify_horizontal_reflection(grid: &Grid, start: usize, end: usize) -> Option<usize> {
    assert_eq!((end - start) % 2, 1);

    // Check correct
    for i in 0..grid.width() {
        if grid[(i, start)] != grid[(i, end)] {
            return None;
        }
    }

    // Return value
    if start == end - 1 {
        Some(start)
    } else {
        verify_horizontal_reflection(grid, start + 1, end - 1)
    }
}

fn verify_vertical_reflection(grid: &Grid, start: usize, end: usize) -> Option<usize> {
    assert_eq!((end - start) % 2, 1);

    // Check correct
    for i in 0..grid.height() {
        if grid[(start, i)] != grid[(end, i)] {
            return None;
        }
    }

    // Return value
    if start == end - 1 {
        Some(start)
    } else {
        verify_vertical_reflection(grid, start + 1, end - 1)
    }
}

fn get_horizontal_reflections(grid: &Grid) -> Vec<ReflectiveLine> {
    let mut v = vec![];
    // From start
    for i in (1..grid.height()).step_by(2) {
        let r = verify_horizontal_reflection(grid, 0, i);
        if r != None {
            v.push(ReflectiveLine::Horizontal(r.unwrap() + 1));
        }
    }
    let start = if grid.height() % 2 == 0 { 0 } else { 1 };
    for i in (start..grid.height()).step_by(2) {
        let r = verify_horizontal_reflection(grid, i, grid.height() - 1);
        if r != None {
            v.push(ReflectiveLine::Horizontal(r.unwrap() + 1));
        }
    }
    v.dedup();
    v
}

fn get_vertical_reflections(grid: &Grid) -> Vec<ReflectiveLine> {
    let mut v = vec![];

    // From start
    for i in (1..grid.width()).step_by(2) {
        let r = verify_vertical_reflection(grid, 0, i);
        if r != None {
            v.push(ReflectiveLine::Vertical(r.unwrap() + 1));
        }
    }
    let start = if grid.width() % 2 == 0 { 0 } else { 1 };
    for i in (start..grid.width()).step_by(2) {
        let r = verify_vertical_reflection(grid, i, grid.width() - 1);
        if r != None {
            v.push(ReflectiveLine::Vertical(r.unwrap() + 1));
        }
    }
    v.dedup();
    v
}

pub struct Task13;

impl TaskCompleter for Task13 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_13/input");
        let mut grids = contents.lines().fold(vec![Grid::new()], |mut v, x| {
            if x.is_empty() {
                v.push(Grid::new());
            } else {
                v.last_mut()
                    .unwrap()
                    .add_line(x.chars().collect::<Vec<char>>())
            };
            v
        });

        grids
            .iter_mut()
            .map(|x| {
                vec![get_vertical_reflections(x), get_horizontal_reflections(x)]
                    .into_iter()
                    .flatten()
            })
            .flatten()
            .map(|x| x.get_final_value())
            .sum::<usize>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_13/input");
        let mut grids = contents.lines().fold(vec![Grid::new()], |mut v, x| {
            if x.is_empty() {
                v.push(Grid::new());
            } else {
                v.last_mut()
                    .unwrap()
                    .add_line(x.chars().collect::<Vec<char>>())
            };
            v
        });

        grids
            .iter_mut()
            .map(|grid| {
                (
                    grid.get_smudges(),
                    // Get the original reflective line
                    vec![get_vertical_reflections(grid), get_horizontal_reflections(grid)]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<ReflectiveLine>>()[0],
                )
            })
            .map(|(grids, v)| {
                let all_reflective_lines_for_grid = grids
                    .iter()
                    .map(|y| {
                        get_vertical_reflections(y)
                            .into_iter()
                            .chain(get_horizontal_reflections(y).into_iter())
                            .filter(|x| x != &v)
                            .collect::<Vec<ReflectiveLine>>()
                    })
                    .flatten()
                    .map(|x| x.get_final_value())
                    .collect::<Vec<usize>>();
                // println!("The grid is {}", x[0]);
                // Make sure all the reflective lines are the same
                all_reflective_lines_for_grid.into_iter()
                    .fold(None, |x, y| match x {
                        Some(t) => {
                            if t == y {
                                Some(t)
                            } else {
                                panic!("ALl new reflective lines must have same value")
                            }
                        }
                        None => Some(y),
                    })
                    .unwrap()
            })
            .sum::<usize>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("33356".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("28475".to_owned())
    }
}
