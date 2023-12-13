use std::{collections::HashSet, iter, ops::Index, str::Chars};

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FloodFillRes {
    NotFilled,
    Outside,
    Inside,
    Path,
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
    fn get_next(&self, from: Direction) -> Option<(Direction, Vec<(i64, i64)>, Vec<(i64, i64)>)> {
        match from {
            Direction::North => match self {
                Tile::VerticalPipe => Some((
                    Direction::North,
                    vec![(-1, -1), (-1, 0), (-1, 1)],
                    vec![(1, -1), (1, 0), (1, 1)],
                )),
                Tile::StoW => Some((
                    Direction::West,
                    vec![(-1, -1)],
                    vec![(-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)],
                )),
                Tile::StoE => Some((
                    Direction::East,
                    vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)],
                    vec![(1, -1)],
                )),
                _ => None,
            },
            Direction::South => match self {
                Tile::VerticalPipe => Some((
                    Direction::South,
                    vec![(1, -1), (1, 0), (1, 1)],
                    vec![(-1, -1), (-1, 0), (-1, 1)],
                )),
                Tile::NtoW => Some((
                    Direction::West,
                    vec![(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)],
                    vec![(-1, 1)],
                )),
                Tile::NtoE => Some((
                    Direction::East,
                    vec![(1, 1)],
                    vec![(-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)],
                )),
                _ => None,
            },
            Direction::East => match self {
                Tile::HorizontalPipe => Some((
                    Direction::East,
                    vec![(-1, 1), (0, 1), (-1, 1)],
                    vec![(-1, -1), (0, -1), (1, -1)],
                )),
                Tile::NtoW => Some((
                    Direction::North,
                    vec![(-1, 1)],
                    vec![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)],
                )),
                Tile::StoW => Some((
                    Direction::South,
                    vec![(-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)],
                    vec![(-1, -1)],
                )),
                _ => None,
            },
            Direction::West => match self {
                Tile::HorizontalPipe => Some((
                    Direction::West,
                    vec![(-1, -1), (0, -1), (1, -1)],
                    vec![(-1, 1), (0, 1), (1, 1)],
                )),
                Tile::NtoE => Some((
                    Direction::North,
                    vec![(1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)],
                    vec![(1, 1)],
                )),
                Tile::StoE => Some((
                    Direction::South,
                    vec![(1, -1)],
                    vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)],
                )),
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

    fn get_loop(&self) -> Option<(Vec<(i64, i64)>, HashSet<(i64, i64)>, HashSet<(i64, i64)>)> {
        let mut res = None;
        for direction in [
            Direction::East,
            Direction::North,
            Direction::South,
            Direction::West,
        ] {
            let mut left_side = HashSet::new();
            let mut right_side = HashSet::new();
            let mut v = vec![];
            let mut pos = direction.next(self.animal_position);
            let mut dir = direction;
            // println!("Going {:?}", dir);
            while pos != self.animal_position {
                v.push(pos);
                if let Some(tile) = self.index_checked(pos.0, pos.1) {
                    if let Some((d, left, right)) = tile.get_next(dir) {
                        left_side.extend(left.iter().map(|(x, y)| (pos.0 + x, pos.1 + y)));
                        right_side.extend(right.iter().map(|(x, y)| (pos.0 + x, pos.1 + y)));

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
                res = Some((v, left_side, right_side));
                break;
            }
        }
        res
    }
}

fn do_flood_fill(grid: &mut Vec<Vec<FloodFillRes>>, centre: (i64, i64), to: FloodFillRes) {
    let offsets = [
        (0, 1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 0),
        (-1, -1),
    ];
    let mut queue = vec![centre];

    while let Some((i, j)) = queue.pop() {
        if grid[j as usize][i as usize] == FloodFillRes::NotFilled {
            grid[j as usize][i as usize] = to;
            queue.append(
                &mut offsets
                    .iter()
                    .map(|(x, y)| (x + i, y + j))
                    .filter(|(x, y)| in_bounds((*x, *y), grid[0].len() as i64, grid.len() as i64))
                    .collect::<Vec<(i64, i64)>>(),
            );
        }
    }
}

fn print_line(g: &Vec<(i64, i64)>, width: i64, height: i64) -> String {
    let mut v = Vec::from_iter(
        iter::repeat(Vec::from_iter(iter::repeat(" ").take(width as usize))).take(height as usize),
    );
    for (x, y) in g {
        v[*y as usize][*x as usize] = "x";
    }
    v.reverse();
    v.iter()
        .map(|x| x.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

fn print_grid(g: &Vec<Vec<FloodFillRes>>, print: FloodFillRes) -> String {
    let mut s = g
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| {
                    if *y == print {
                        "X".to_owned()
                    } else {
                        " ".to_owned()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>();
    s.reverse();
    s.join("\n")
}

fn in_bounds(pos: (i64, i64), width: i64, height: i64) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
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

        let l = grid.get_loop().unwrap().0.len();

        // Answer is half way around the loop
        format!("{:?}", l / 2 + 1)
    }

    fn do_task_2(&self) -> String {
        let contents = include_str!("../input/day_10/input");
        let grid = Grid::new(contents);
        assert_eq!(grid[grid.animal_position], Tile::Animal);

        let (path, left_side, right_side) = grid.get_loop().unwrap();
        let mut flood_fill: Vec<Vec<FloodFillRes>> = grid
            .grid
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| FloodFillRes::NotFilled)
                    .collect::<Vec<FloodFillRes>>()
            })
            .collect();
        flood_fill[grid.animal_position.1 as usize][grid.animal_position.0 as usize] =
            FloodFillRes::Path;
        for (x, y) in &path {
            flood_fill[*y as usize][*x as usize] = FloodFillRes::Path;
        }

        for y in 0..flood_fill.len() {
            if flood_fill[y][0] != FloodFillRes::Path {
                do_flood_fill(&mut flood_fill, (0 as i64, y as i64), FloodFillRes::Outside);
            }
            let l = flood_fill[0].len() - 1;
            if flood_fill[y][l] != FloodFillRes::Path {
                do_flood_fill(&mut flood_fill, (l as i64, y as i64), FloodFillRes::Outside);
            }
        }

        for x in 0..flood_fill[0].len() {
            if flood_fill[0][x] != FloodFillRes::Path {
                do_flood_fill(&mut flood_fill, (x as i64, 0), FloodFillRes::Outside);
            }
            let l = flood_fill.len() - 1;
            if flood_fill[l][x] != FloodFillRes::Path {
                do_flood_fill(&mut flood_fill, (x as i64, l as i64), FloodFillRes::Outside);
            }
        }

        let mut flood_fill_left = flood_fill;
        let mut flood_fill_right = flood_fill_left.clone();
        let mut use_left_side = true;

        for pos in left_side
            .iter()
            .filter(|x| in_bounds(**x, grid.grid[0].len() as i64, grid.grid.len() as i64))
        {
            if flood_fill_left[pos.1 as usize][pos.0 as usize] == FloodFillRes::Outside {
                use_left_side = false;
                break;
            } else if flood_fill_left[pos.1 as usize][pos.0 as usize] == FloodFillRes::NotFilled {
                do_flood_fill(&mut flood_fill_left, *pos, FloodFillRes::Inside);
            }
        }
        if use_left_side {
            flood_fill_left
                .iter()
                .flatten()
                .filter(|x| **x == FloodFillRes::Inside)
                .count()
                .to_string()
        } else {
            for pos in right_side
                .iter()
                .filter(|x| in_bounds(**x, grid.grid[0].len() as i64, grid.grid.len() as i64))
            {
                if flood_fill_right[pos.1 as usize][pos.0 as usize] == FloodFillRes::Outside {
                    // println!("{}", print_grid(&flood_fill_right, FloodFillRes::Outside));
                    panic!["Both sides touch outside, seen at pos: {:?}", pos];
                } else if flood_fill_right[pos.1 as usize][pos.0 as usize]
                    == FloodFillRes::NotFilled
                {
                    do_flood_fill(&mut flood_fill_right, *pos, FloodFillRes::Inside);
                }
            }
            flood_fill_right
                .iter()
                .flatten()
                .filter(|x| **x == FloodFillRes::Inside)
                .count()
                .to_string()
        }
    }

    fn task_1_result(&self) -> Option<String> {
        Some("6754".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("567".to_owned())
    }
}
