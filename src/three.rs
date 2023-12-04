use std::{
    ops::{Index, IndexMut},
    rc::Rc,
};

use crate::helpers::get_contents;

struct Grid {
    size: (usize, usize),
    cells: Vec<Cell>,
}

impl Grid {
    fn create(input: &str) -> Self {
        let lines: Vec<&str> = input.split("\n").collect();
        let height = lines.len();
        let width = lines[0].len();
        let cells = Vec::with_capacity(height * width);

        Grid {
            size: (width, height),
            cells,
        }
    }
}

enum Cell {
    Dot,
    Punctuation,
    Number(Rc<u32>),
}

pub fn run_task_1() {
    let grid = Grid::create(&get_contents("three".to_owned()));
}
