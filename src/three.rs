use std::collections::HashSet;

use crate::TaskCompleter;

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<GridCell>,
    numbers: Vec<u32>,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
struct NumberID(usize);

impl Grid {
    fn create(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut cells = Vec::with_capacity(height * width);
        let mut numbers = Vec::new();

        for line in lines {
            assert_eq!(line.len(), width);
            let mut number = 0;
            for char in line.chars() {
                cells.push(match char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        // Stinky but I don't care
                        number *= 10;
                        number += char.to_digit(10).unwrap();
                        GridCell::Number(NumberID(numbers.len()))
                    }
                    '*' => {
                        if number != 0 {
                            numbers.push(number);
                            number = 0;
                        }
                        GridCell::Gear
                    }
                    '.' => {
                        if number != 0 {
                            numbers.push(number);
                            number = 0;
                        }
                        GridCell::Dot
                    }
                    _ => {
                        if number != 0 {
                            numbers.push(number);
                            number = 0;
                        }
                        GridCell::Punctuation
                    }
                })
            }
            if number != 0 {
                numbers.push(number);
            }
        }
        Grid {
            width,
            height,
            cells,
            numbers,
        }
    }

    fn get_index(&self, height: usize, width: usize) -> &GridCell {
        &self.cells[(height * self.width) + width]
    }

    fn get_sum(&mut self) -> (u32, u32) {
        let mut numbers = HashSet::new();
        let mut gear_sum = 0;
        for height in 0..self.height {
            for width in 0..self.width {
                match self.get_index(height, width) {
                    GridCell::Gear => {
                        let mut neighbours = Vec::new();
                        self.get_neighbours(height, width).for_each(|x| {
                            match x {
                                GridCell::Number(n) => {
                                    neighbours.push(*n);
                                    numbers.insert(*n);
                                }
                                _ => (),
                            };
                        });
                        neighbours.sort();
                        neighbours.dedup();
                        if neighbours.len() == 2 {
                            gear_sum +=
                                self.numbers[neighbours[0].0] * self.numbers[neighbours[1].0];
                        }
                    }
                    GridCell::Punctuation => self.get_neighbours(height, width).for_each(|x| {
                        match x {
                            GridCell::Number(n) => {
                                numbers.insert(*n);
                            }
                            _ => (),
                        };
                    }),
                    _ => (),
                };
            }
        }
        (
            numbers.iter().map(|x| self.numbers[x.0]).sum::<u32>(),
            gear_sum,
        )
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
    Gear,
    Number(NumberID),
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

pub struct Task3;

impl TaskCompleter for Task3 {
    fn do_task_1(&self) -> String {
        let mut grid = Grid::create(include_str!("../input/three/input"));

        let (sum, _gear_sum) = grid.get_sum();

        sum.to_string()
    }

    fn do_task_2(&self) -> String {
        let mut grid = Grid::create(include_str!("../input/three/input"));

        let (_sum, gear_sum) = grid.get_sum();

        gear_sum.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("546312".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("87449461".to_owned())
    }
}
