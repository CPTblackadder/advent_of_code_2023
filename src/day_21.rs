use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Coord, Direction, Grid},
    TaskCompleter,
};

pub struct Task21;

fn get_accesible_tiles(
    location: Coord,
    steps: i64,
    grid: &Grid<char>,
    cache: &mut HashMap<(Coord, i64), HashSet<Coord>>,
) -> HashSet<Coord> {
    if let Some(h) = cache.get(&(location, steps)) {
        h.clone()
    } else if steps == 0 {
        let mut h = HashSet::new();
        h.insert(location);
        h
    } else {
        let hash = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(|dir| {
            if let Some(new_l) = location.translate(dir, grid) {
                if grid[new_l] != '#' {
                    get_accesible_tiles(new_l, steps - 1, grid, cache)
                } else {
                    HashSet::new()
                }
            } else {
                HashSet::new()
            }
        })
        .fold(HashSet::new(), |mut v, h| {
            v.extend(h);
            v
        });
        cache.insert((location, steps), hash.clone());
        hash
    }
}

impl TaskCompleter for Task21 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_21/input");
        let g = Grid::from_string(contents, false);
        let starting_location = g.find_coord(|x| x == &'S').unwrap();
        let mut cache = HashMap::new();

        let tiles = get_accesible_tiles(starting_location, 64, &g, &mut cache);
        tiles.len().to_string()
    }

    fn do_task_2(&self) -> String {
        "Todo".to_owned()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("3724".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
