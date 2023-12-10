use std::ops::Index;

use crate::TaskCompleter;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NtoE,
    NtoW,
    StoW,
    StoE,
    Ground,
    Animal,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn next(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
}

impl Tile {
    fn get_next(&self, from: Direction) -> Option<Direction> {
        match from {
            Direction::North => match self {
                Tile::VerticalPipe => Some(Direction::North),
                Tile::StoW => Some(Direction::West),
                Tile::StoE => Some(Direction::East),
                _ => None,
            },
            Direction::South => match self {
                Tile::VerticalPipe => Some(Direction::South),
                Tile::NtoW => Some(Direction::West),
                Tile::NtoE => Some(Direction::East),
                _ => None,
            },
            Direction::East => match self {
                Tile::HorizontalPipe => Some(Direction::East),
                Tile::NtoW => Some(Direction::North),
                Tile::StoW => Some(Direction::South),
                _ => None,
            },
            Direction::West => match self {
                Tile::HorizontalPipe => Some(Direction::West),
                Tile::NtoE => Some(Direction::North),
                Tile::StoE => Some(Direction::South),
                _ => None,
            },
        }
    }
}

pub struct Grid {
    grid: Vec<Vec<Tile>>,
    animal_position: (i64, i64),
}

impl Index<(i64, i64)> for Grid {
    type Output = Tile;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        &self.grid[index.1 as usize][index.0 as usize]
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines = input.lines();
        let mut grid = vec![];
        let mut starting_position = None;
        for line in lines {
            let mut grid_line = vec![];
            for char in line.chars() {
                grid_line.push(match char {
                    '|' => Tile::VerticalPipe,
                    '-' => Tile::HorizontalPipe,
                    'L' => Tile::NtoE,
                    'J' => Tile::NtoW,
                    '7' => Tile::StoW,
                    'F' => Tile::StoE,
                    '.' => Tile::Ground,
                    'S' => Tile::Animal,
                    _ => panic!("Invalid tile"),
                });
                if let Some(Tile::Animal) = grid_line.last() {
                    // Horizontal first, vertical second
                    starting_position = Some((grid_line.len() - 1, grid.len()))
                }
            }
            grid.push(grid_line);
        }
        grid.reverse();
        if let Some((x, y)) = starting_position {
            starting_position = Some((x, grid.len() - 1 - y));
        } else {
            panic!("No Starting animal");
        }
        Self {
            grid,
            animal_position: (
                starting_position.unwrap().0 as i64,
                starting_position.unwrap().1 as i64,
            ),
        }
    }

    pub fn index_checked(&self, x: i64, y: i64) -> Option<Tile> {
        if x < 0 || y < 0 || x as usize >= self.grid[0].len() || y as usize >= self.grid.len() {
            None
        } else {
            Some(self.grid[y as usize][x as usize])
        }
    }

    fn get_loop(&self) -> Option<Vec<(i64, i64)>> {
        let mut res = None;
        for direction in [
            Direction::East,
            Direction::North,
            Direction::South,
            Direction::West,
        ] {
            let mut v = vec![];
            let mut pos = direction.next(self.animal_position);
            let mut dir = direction;
            println!("Going {:?}", dir);
            while pos != self.animal_position {
                v.push(pos);
                if let Some(tile) = self.index_checked(pos.0, pos.1) {
                    if let Some(d) = tile.get_next(dir) {
                        dir = d;
                        pos = dir.next(pos);
                    } else {
                        // This loop ends here
                        //println!("Loop ends due to tile {:?} not accepting direction {:?}", tile, dir);
                        break;
                    }
                } else {
                    // This loop ends here
                    // println!("Loop ends due to tile index {:?} being out of bounds", pos);
                    break;
                }
            }
            if pos == self.animal_position {
                // Found loop
                res = Some(v);
                break;
            }
        }
        res
    }
}

pub struct Task10;

impl TaskCompleter for Task10 {
    fn get_name(&self) -> String {
        "10".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/day_10/input");
        let grid = Grid::new(contents);
        assert_eq!(grid[grid.animal_position], Tile::Animal);

        let l = grid.get_loop().unwrap().len();

        // Answer is half way around the loop
        format!("{:?}", l / 2 + 1)
    }

    fn do_task_2(&self) -> String {
        let contents = include_str!("../input/day_10/input");
        let grid = Grid::new(contents);
        assert_eq!(grid[grid.animal_position], Tile::Animal);

        let l = grid.get_loop().unwrap();



        "todo".to_owned()
    }
}
