use core::panic;

use crate::TaskCompleter;

struct Game {
    id: u32,
    highest_value: MaxValueMap,
}

impl Game {
    fn create(input: &str) -> Self {
        let mut input = input.split(":");
        let game_id_str = input.next().unwrap();
        let game_run = input.next().unwrap();
        let id;
        if game_id_str.starts_with("Game ") {
            id = game_id_str[5..].parse::<u32>().unwrap();
        } else {
            panic!("Invalid format");
        }

        let mut highest_value = MaxValueMap::default();
        for picked_colours in game_run.split(";") {
            for colour in picked_colours.split(",") {
                let mut colour_itr = colour.split(" ");
                colour_itr.next();
                let number = colour_itr.next().unwrap().parse::<u32>().unwrap();
                let colour = colour_itr.next().unwrap();
                match colour {
                    "red" => highest_value.update(Colour::Red, number),
                    "green" => highest_value.update(Colour::Green, number),
                    "blue" => highest_value.update(Colour::Blue, number),
                    _ => panic!("Invalid colour"),
                }
            }
        }

        Self {
            id: id,
            highest_value: highest_value,
        }
    }
}

#[derive(Default, PartialEq, Debug)]
struct MaxValueMap {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialOrd for MaxValueMap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.red.partial_cmp(&other.red) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(core::cmp::Ordering::Less) => {}
            ord => return ord,
        }
        match self.green.partial_cmp(&other.green) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(core::cmp::Ordering::Less) => {}
            ord => return ord,
        }
        self.blue.partial_cmp(&other.blue)
    }
}

impl MaxValueMap {
    fn update(&mut self, colour: Colour, number: u32) {
        match colour {
            Colour::Red => {
                if self.red < number {
                    self.red = number
                }
            }
            Colour::Green => {
                if self.green < number {
                    self.green = number
                }
            }
            Colour::Blue => {
                if self.blue < number {
                    self.blue = number
                }
            }
        }
    }

    fn get_power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

enum Colour {
    Red,
    Green,
    Blue,
}

pub struct Task2;

impl TaskCompleter for Task2 {
    fn do_task_1(&self) -> String {
        let contents: Vec<u32> = include_str!("../input/two/input")
            .split("\n")
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(Game::create(x))
                }
            })
            .filter(|x| {
                &x.highest_value
                    <= &MaxValueMap {
                        red: 12,
                        green: 13,
                        blue: 14,
                    }
            })
            .map(|x| x.id)
            .collect();
        contents.iter().sum::<u32>().to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: Vec<u32> = include_str!("../input/two/input")
            .split("\n")
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(Game::create(x))
                }
            })
            .map(|x| x.highest_value.get_power())
            .collect();
        contents.iter().sum::<u32>().to_string()
    }
    fn task_1_result(&self) -> Option<String> {
        Some("2162".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("72513".to_owned())
    }
}
