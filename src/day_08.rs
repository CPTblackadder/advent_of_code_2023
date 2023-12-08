use std::{collections::HashMap, iter::zip, thread};

use crate::TaskCompleter;

pub struct Task8;

pub fn str_to_index(input: &str) -> u32 {
    assert_eq!(input.len(), 3);
    let mut i = 0;
    for (mul, char) in zip(1.., input.chars()) {
        i += (26_u32.pow(mul)) * (char as u32 - 'A' as u32);
    }
    i
}

pub fn do_task_2_original() -> String {
    let contents = include_str!("../input/day_08/input");
    let mut lines = contents.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    // Empty line
    lines.next();
    let mut mapping = HashMap::<&str, (&str, &str)>::new();
    for line in lines {
        mapping.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }
    let mut spaces: Vec<&str> = mapping
        .iter()
        .filter_map(|(k, _)| if k.ends_with("A") { Some(*k) } else { None })
        .collect();
    let mut count = 0;
    while !spaces.iter().all(|x| x.ends_with("Z")) {
        let dir = instructions[count % instructions.len()];
        for x in spaces.iter_mut() {
            *x = match dir {
                'L' => mapping[x].0,
                'R' => mapping[x].1,
                _ => panic!("Invalid character {}", dir),
            }
        }
        count += 1;
    }
    count.to_string()
}

pub fn do_task_2_maybe_quicker() -> String {
    let builder = thread::Builder::new().stack_size(32 * 26_usize.pow(4));

    let handler = builder
        .spawn(|| {
            let contents = include_str!("../input/day_08/input");
            let mut lines = contents.lines();
            let instructions: Vec<char> = lines.next().unwrap().chars().collect();
            // Empty line
            lines.next();
            let mut mapping = HashMap::<&str, (&str, &str)>::new();
            for line in lines {
                mapping.insert(&line[0..3], (&line[7..10], &line[12..15]));
            }
            let mut spaces: Vec<u32> = mapping
                .iter()
                .filter_map(|(k, _)| if k.ends_with("A") { Some(*k) } else { None })
                .map(str_to_index)
                .collect();
            let final_spaces: Vec<u32> = mapping
                .iter()
                .filter_map(|(k, _)| if k.ends_with("Z") { Some(*k) } else { None })
                .map(str_to_index)
                .collect();
            let hash_mapping: HashMap<u32, (u32, u32)> = mapping
                .into_iter()
                .map(|(k, (l, r))| (str_to_index(k), (str_to_index(l), str_to_index(r))))
                .collect();
            let mut mapping = [(0, 0); 26_usize.pow(4)];
            for (k, v) in hash_mapping {
                mapping[k as usize] = v;
            }
            let mut count = 0;
            while !spaces.iter().all(|x| final_spaces.contains(x)) {
                let dir = instructions[count % instructions.len()];
                for x in &mut spaces.iter_mut() {
                    *x = match dir {
                        'L' => mapping[*x as usize].0,
                        'R' => mapping[*x as usize].1,
                        _ => panic!("Invalid character {}", dir),
                    }
                }
                count += 1;
            }
            count.to_string()
        })
        .unwrap();
    handler.join().unwrap()
}

fn do_task_2_maybe_smarter() -> String {
    let contents = include_str!("../input/day_08/input");
    let mut lines = contents.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    // Empty line
    lines.next();
    let mut mapping = HashMap::<&str, (&str, &str)>::new();
    for line in lines {
        mapping.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    let spaces: Vec<&str> = mapping
        .iter()
        .filter_map(|(k, _)| if k.ends_with("A") { Some(*k) } else { None })
        .collect();

    let first_cycle: Vec<(&str, &str, usize)> = dbg!(spaces)
        .iter()
        .map(|x| {
            let mut count = 0;
            let mut space = *x;
            let starting_value = x.clone();
            while !space.ends_with("Z") {
                let dir = instructions[count % instructions.len()];
                space = match dir {
                    'L' => mapping[space].0,
                    'R' => mapping[space].1,
                    _ => panic!("Invalid character {}", dir),
                };
                count += 1;
            }
            (starting_value, *x, count)
        })
        .collect();

    let mut all_subsequent_cycles: Vec<(&str, usize, usize, usize)> = dbg!(first_cycle)
        .iter()
        .map(|(x, starting_value, initial_cycle_count)| {
            let mut count = 0;
            let mut space = *x;
            while !space.ends_with("Z") {
                let dir = instructions[count % instructions.len()];
                space = match dir {
                    'L' => mapping[space].0,
                    'R' => mapping[space].1,
                    _ => panic!("Invalid character {}", dir),
                };
                count += 1;
            }
            (
                *starting_value,
                *initial_cycle_count,
                count,
                *initial_cycle_count,
            )
        })
        .collect();

    // Find a number where all initial_cycle_count + count is equal accross all different starting spaces
    all_subsequent_cycles
        .iter()
        .fold(1, |x, y| num::integer::lcm(x, y.1))
        .to_string()
}

impl TaskCompleter for Task8 {
    fn get_name(&self) -> String {
        "8".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/day_08/input");
        let mut lines = contents.lines();
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        // Empty line
        lines.next();
        let mut mapping = HashMap::<&str, (&str, &str)>::new();
        for line in lines {
            mapping.insert(&line[0..3], (&line[7..10], &line[12..15]));
        }
        let mut space = "AAA";
        let mut count = 0;
        while space != "ZZZ" {
            let dir = instructions[count % instructions.len()];
            space = match dir {
                'L' => mapping[space].0,
                'R' => mapping[space].1,
                _ => panic!("Invalid character {}", dir),
            };
            count += 1;
        }
        count.to_string()
    }

    fn do_task_2(&self) -> String {
        do_task_2_maybe_smarter()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::day_08::str_to_index;

    #[test]
    fn str_to_index_test() {
        let mut seen_indices: HashMap<u32, String> = HashMap::new();

        for char1 in 'A'..'Z' {
            for char2 in 'A'..'Z' {
                for char3 in 'A'..'Z' {
                    let str = format!("{}{}{}", char1, char2, char3);
                    let index = str_to_index(&str);
                    assert!(
                        !seen_indices.contains_key(&index),
                        "Seen index {} before, corresponding string: {} This clashes with: {}",
                        index,
                        str,
                        seen_indices[&index],
                    );
                    seen_indices.insert(index, str);
                }
            }
        }
    }
}
