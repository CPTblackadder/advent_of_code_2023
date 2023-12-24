use std::iter::zip;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator};

use crate::TaskCompleter;

pub struct Task24;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}
impl Position {
    fn from_str(input: &str) -> Self {
        let (x, y, z) = get_3_f64_from_str(input);
        Self { x, y, z }
    }

    fn translate(&self, vel: Velocity, time: f64) -> Self {
        Self {
            x: self.x + (vel.x * time),
            y: self.y + (vel.y * time),
            z: self.z + (vel.z * time),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

impl Velocity {
    fn from_str(input: &str) -> Self {
        let (x, y, z) = get_3_f64_from_str(input);
        Self { x, y, z }
    }

    fn is_parralel_2d(&self, other: &Self) -> bool {
        (self.x / other.x) == (self.y / other.y)
    }

    fn is_parralel_3d(&self, other: &Self) -> bool {
        (self.x / other.x) == (self.y / other.y)
            && (self.x / other.x) == (self.z / other.z)
            && (self.y / other.y) == (self.z / other.z)
    }
}

fn get_3_f64_from_str(input: &str) -> (f64, f64, f64) {
    let mut s = input.split(",");
    let x = s.next().unwrap().trim().parse::<f64>().unwrap();
    let y = s.next().unwrap().trim().parse::<f64>().unwrap();
    let z = s.next().unwrap().trim().parse::<f64>().unwrap();
    (x, y, z)
}

#[derive(Debug, PartialEq)]
struct Hailstone {
    pos: Position,
    vel: Velocity,
}

impl Hailstone {
    fn from_str(input: &str) -> Self {
        let mut s = input.split("@");
        let pos = Position::from_str(s.next().unwrap());
        let vel = Velocity::from_str(s.next().unwrap());
        Self { pos, vel }
    }

    fn intersects_at_2d(&self, other: &Self) -> Option<(f64, f64)> {
        if self.vel.is_parralel_2d(&other.vel) {
            return None;
        }
        let (a1, b1, c1) = self.get_general_form_equation_2d();
        let (a2, b2, c2) = other.get_general_form_equation_2d();
        let x = ((b1 * c2) - (b2 * c1)) / ((a1 * b2) - (a2 * b1));
        let y = ((c1 * a2) - (c2 * a1)) / ((a1 * b2) - (a2 * b1));
        // Check if crossed in past
        if self.in_future_2d((x, y)) && other.in_future_2d((x, y)) {
            Some((x, y))
        } else {
            None
        }
    }

    fn in_future_2d(&self, point: (f64, f64)) -> bool {
        self.vel.x.is_sign_positive() && self.pos.x < point.0
            || self.vel.x.is_sign_negative() && self.pos.x > point.0
    }

    fn get_general_form_equation_2d(&self) -> (f64, f64, f64) {
        let pos1 = self.pos;
        let pos2 = self.pos.translate(self.vel, 1f64);
        let a = -(pos2.y - pos1.y);
        let b = pos2.x - pos1.x;
        let c = -((a * pos1.x) + (b * pos1.y));
        (a, b, c)
    }
}

impl TaskCompleter for Task24 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_24/input");
        let hailstones = contents
            .lines()
            .map(Hailstone::from_str)
            .collect::<Vec<Hailstone>>();
        const TEST_RANGE_MIN: f64 = 200000000000000f64;
        const TEST_RANGE_MAX: f64 = 400000000000000f64;

        zip(0.., hailstones.iter())
            .map(|(i, h)| {
                hailstones[i..]
                    .iter()
                    .filter_map(|h2| h.intersects_at_2d(h2))
                    .filter(|(x, y)| {
                        x >= &TEST_RANGE_MIN
                            && x <= &TEST_RANGE_MAX
                            && y >= &TEST_RANGE_MIN
                            && y <= &TEST_RANGE_MAX
                    })
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_24/input");
        let hailstones = contents
            .lines()
            .map(Hailstone::from_str)
            .collect::<Vec<Hailstone>>();

        let parallel = zip(0.., hailstones.iter())
            .map(|(i, h)| {
                (
                    h,
                    hailstones[i..]
                        .iter()
                        .filter(|h2| &h != h2 && h.vel.is_parralel_3d(&h2.vel))
                        .collect::<Vec<&Hailstone>>(),
                )
            })
            .filter(|(_, v)| !v.is_empty())
            .collect::<Vec<(&Hailstone, Vec<&Hailstone>)>>();

        dbg!(parallel);

        "Todo".to_owned()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
