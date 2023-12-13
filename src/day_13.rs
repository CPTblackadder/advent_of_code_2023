use std::ops::Index;

use crate::TaskCompleter;

#[derive(PartialEq, Eq)]
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
}

struct Grid {
    g: Vec<Vec<Tile>>,
    rotated: bool,
}

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if self.rotated {
            &self.g[index.0][index.1]
        } else {
            &self.g[index.1][index.0]
        }
    }
}

impl Grid {
    fn width(&self) -> usize {
        if self.rotated {
            self.g.len()
        } else {
            self.g[0].len()
        }
    }
    fn height(&self) -> usize {
        if self.rotated {
            self.g[0].len()
        } else {
            self.g.len()
        }
    }

    fn new() -> Self {
        Grid {
            g: vec![],
            rotated: false,
        }
    }

    fn add_line(&mut self, char: Vec<char>) {
        self.g.push(
            char.into_iter()
                .map(|x| Tile::new(x))
                .collect::<Vec<Tile>>(),
        );
    }
}

fn verify_hoirzontal_reflection(grid: &Grid, start: usize, end: usize) -> Option<usize> {
    println!("Start: {}, End: {}", start, end);
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
        verify_hoirzontal_reflection(grid, start + 1, end - 1)
    }
}

fn get_horizontal_reflection(grid: &Grid) -> Option<usize> {
    // From start
    for i in (1..grid.height()).step_by(2) {
        let r = verify_hoirzontal_reflection(grid, 0, i);
        if r != None {
            return r;
        }
    }
    let start = if grid.height() % 2 == 0 { 0 } else { 1 };
    for i in (start..grid.height()).step_by(2) {
        let r = verify_hoirzontal_reflection(grid, i, grid.height());
        if r != None {
            return r;
        }
    }

    None
}

pub struct Task13;

impl TaskCompleter for Task13 {
    fn get_name(&self) -> String {
        "12".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_13/input");
        let grids = contents
            .lines()
            .fold(vec![Grid::new()], |mut v, x| {
                if x.is_empty() {
                    v.push(Grid::new());
                } else {
                    v.last_mut()
                        .unwrap()
                        .add_line(x.chars().collect::<Vec<char>>())
                };
                v
            });
        let results = grids
            .iter()
            .map(get_horizontal_reflection);
        format!("{:?}", results.collect::<Vec<Option<usize>>>())
    }

    fn do_task_2(&self) -> String {
        "todo!()".to_owned()
    }
}
