use crate::{
    grid::{Coord, Direction, Grid},
    TaskCompleter,
};
use crossbeam::deque::{Steal, Worker};
use rayon::spawn;
use std::{
    collections::VecDeque,
    rc::Rc,
    sync::{
        atomic::{self, AtomicUsize},
        mpsc, Arc, Mutex,
    },
    thread,
};

pub struct Task23;

fn get_longest_path(grid: &Grid<char>, start_tile: Coord, end_tile: Coord) -> i64 {
    let mut steps = Grid::init_with_size(-1, grid.width(), grid.height());
    let mut queue = VecDeque::new();
    queue.push_front((start_tile, Direction::Up, 0));
    while let Some((tile, dir, s)) = queue.pop_back() {
        if grid[tile] == '#' || steps[tile] >= s {
            continue;
        }
        steps[tile] = s;
        if tile == end_tile {
            continue;
        }
        let dirs = match grid[tile] {
            '.' => vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
            '>' => vec![Direction::Left],
            '<' => vec![Direction::Right],
            '^' => vec![Direction::Down],
            'v' => vec![Direction::Up],
            _ => panic!("Invalid characted: {}", grid[tile]),
        };
        for d in dirs {
            if d == dir.opposite() {
                continue;
            }
            queue.push_front((tile.translate_no_bounds(d), d, s + 1));
        }
    }
    steps[end_tile]
}

fn get_longest_path_2(grid: &Grid<char>, start_tile: Coord, end_tile: Coord) -> i64 {
    let mut queue = VecDeque::new();
    let mut finishing_steps = 0;
    queue.push_front((
        start_tile,
        Grid::init_with_size(false, grid.width(), grid.height()),
        0,
    ));
    while let Some((tile, mut steps, s)) = queue.pop_back() {
        if tile == end_tile {
            if finishing_steps < s {
                finishing_steps = s;
                // println!("Finished with {} steps", s);
            }
            continue;
        }
        steps[tile] = true;
        let dirs = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|d| tile.translate(d, grid))
        .filter(|t| grid[*t] != '#' && !steps[*t])
        .collect::<Vec<Coord>>();
        if dirs.len() > 1 {
            for t in &dirs[1..] {
                queue.push_back((*t, steps.clone(), s + 1));
            }
        }
        if dirs.len() > 0 {
            queue.push_back((dirs[0], steps, s + 1));
        }
    }
    finishing_steps
}

fn get_longest_path_multi_threading(grid: &Grid<char>, start_tile: Coord, end_tile: Coord) -> i64 {
    let (s, r) = crossbeam::channel::unbounded();
    let _ = s.send((
        start_tile,
        Grid::init_with_size(false, grid.width(), grid.height()),
        0,
    ));
    let grid = Arc::new(grid.clone());
    let threads_completed = Arc::new(Mutex::new(Box::new(0)));
    let mut threads = vec![];
    for _ in 0..8 {
        let (send, rec) = (s.clone(), r.clone());
        let mut finishing_steps: i64 = 0;
        let grid = grid.clone();
        let threads_completed = threads_completed.clone();
        threads.push(thread::spawn(move || {
            let mut thread_completed = false;
            loop {
                if let Ok((tile, mut steps, s)) = rec.try_recv() {
                    if thread_completed {
                        thread_completed = false;
                        if let Ok(mut t) = threads_completed.lock() {
                            *t.as_mut() -= 1;
                        }
                    }
                    if tile == end_tile {
                        if finishing_steps < s {
                            finishing_steps = s;
                            // println!("Finished with {} steps", s);
                        }
                        continue;
                    }
                    steps[tile] = true;
                    let dirs = vec![
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .into_iter()
                    .filter_map(|d| tile.translate(d, grid.as_ref()))
                    .filter(|t| grid[*t] != '#' && !steps[*t])
                    .collect::<Vec<Coord>>();
                    if dirs.len() > 1 {
                        for t in &dirs[1..] {
                            let _ = send.send((*t, steps.clone(), s + 1));
                        }
                    }
                    if dirs.len() > 0 {
                        let _ = send.send((dirs[0], steps, s + 1));
                    }
                } else {
                    if let Ok(mut t) = threads_completed.lock() {
                        if !thread_completed {
                            *t.as_mut() += 1;
                        }
                        if t.as_ref() == &8 {
                            return finishing_steps;
                        }
                    }
                    thread_completed = true;
                }
            }
        }));
    }

    threads
        .into_iter()
        .map(|x| x.join().unwrap())
        .max()
        .unwrap()
}

impl TaskCompleter for Task23 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_23/input");
        let g = Grid::from_string(contents, false);
        let start_tile = (0..g.width() as i64)
            .into_iter()
            .find(|x| g[Coord::new(*x, 0)] == '.')
            .unwrap();
        let end_tile = (0..g.width() as i64)
            .into_iter()
            .find(|x| g[Coord::new(*x, g.height() as i64 - 1)] == '.')
            .unwrap();
        get_longest_path(
            &g,
            Coord::new(start_tile, 0),
            Coord::new(end_tile, g.height() as i64 - 1),
        )
        .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_23/input");
        let g: Grid<char> = Grid::from_string(contents, false);
        let start_tile = (0..g.width() as i64)
            .into_iter()
            .find(|x| g[Coord::new(*x, 0)] == '.')
            .unwrap();
        let end_tile = (0..g.width() as i64)
            .into_iter()
            .find(|x| g[Coord::new(*x, g.height() as i64 - 1)] == '.')
            .unwrap();
        get_longest_path_multi_threading(
            &g,
            Coord::new(start_tile, 0),
            Coord::new(end_tile, g.height() as i64 - 1),
        )
        .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("2370".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
