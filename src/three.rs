use core::num;
use std::{
    borrow::BorrowMut,
    cell::Cell,
    collections::HashSet,
    ops::{Index, IndexMut},
    rc::Rc,
};

use crate::helpers::get_contents;

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<GridCell>,
}

impl Grid {
    fn create(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut cells = Vec::with_capacity(height * width);

        for line in lines {
            assert_eq!(line.len(), width);
            let mut number = Rc::new(Cell::new((0, false)));
            for char in line.chars() {
                cells.push(match char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        let n = number.borrow_mut().as_ptr();
                        // Stinky but I don't care
                        unsafe {
                            (*n).0 *= 10;
                            (*n).0 += char.to_digit(10).unwrap()
                        };
                        GridCell::Number(number.clone())
                    }
                    '.' => {
                        number = Rc::new(Cell::new((0, false)));
                        GridCell::Dot
                    }
                    _ => {
                        number = Rc::new(Cell::new((0, false)));
                        GridCell::Punctuation
                    }
                })
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    fn get_index(&self, height: usize, width: usize) -> &GridCell {
        &self.cells[(height * self.width) + width]
    }

    fn get_sum(&mut self) -> (u32, Vec<u32>) {
        let mut numbers_used = Vec::new();
        let mut sum = 0;
        for height in 0..self.height {
            for width in 0..self.width {
                match self.get_index(height, width) {
                    GridCell::Punctuation => self.get_neighbours(height, width).for_each(|x| {
                        match x {
                            GridCell::Number(n) => {
                                if !n.get().1 {
                                    numbers_used.push(n.get().0);
                                    sum += n.get().0;
                                    unsafe {
                                        (*n.as_ptr()).1 = true;
                                    }
                                }
                            }
                            _ => (),
                        };
                    }),
                    _ => (),
                };
            }
        }
        (sum, numbers_used)
    }

    fn get_neighbours<'a>(&'a self, height: usize, width: usize) -> GridCellNeighbours<'a> {
        GridCellNeighbours {
            height,
            width,
            index: 0,
            grid: &self,
        }
    }
}

#[derive(PartialEq, Debug)]
enum GridCell {
    Dot,
    Punctuation,
    Number(Rc<Cell<(u32, bool)>>),
}

struct GridCellNeighbours<'a> {
    height: usize,
    width: usize,
    index: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for GridCellNeighbours<'a> {
    type Item = &'a GridCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 8 {
            return None;
        }
        let mut height;
        let mut width;
        loop {
            let (x, y) = [
                (1, 1),
                (1, 0),
                (1, -1),
                (0, 1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ][self.index];
            height = self.height as i32 + y;
            width = self.width as i32 + x;
            if height >= 0
                && height < self.grid.height as i32
                && width >= 0
                && width < self.grid.width as i32
            {
                self.index += 1;
                break;
            }
            self.index += 1;
            if self.index >= 8 {
                return None;
            }
        }
        Some(self.grid.get_index(height as usize, width as usize))
    }
}

pub fn run_task_1() -> Vec<u32> {
    let mut grid = Grid::create(&get_contents("three".to_owned()));

    let (sum, numbers_used) = grid.get_sum();

    println!("{}", sum);
    numbers_used
}
