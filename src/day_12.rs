use crate::TaskCompleter;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    iter::{self, zip},
    time::{Duration, Instant},
};

fn verify_sequence(sequence: &Vec<char>, verify: &Vec<u32>) -> bool {
    let mut verify_sequence = None;
    let mut verify_index = 0;
    for i in sequence {
        match *i {
            '#' => {
                if verify_index == verify.len() {
                    return false;
                };
                verify_sequence = match verify_sequence {
                    Some(x) => Some(x + 1),
                    None => Some(1),
                };
            }
            '.' => match verify_sequence {
                Some(x) => {
                    if verify_index >= verify.len() || x != verify[verify_index] {
                        return false;
                    } else {
                        verify_index += 1;
                        verify_sequence = None;
                    }
                }
                None => (),
            },
            _ => panic!("Unknown charachter {} while verifying", i),
        }
    }
    verify_index == verify.len()
}

fn get_next_symbol(symbol: char, from: usize, sequence: &Vec<char>) -> Option<usize> {
    for i in from..sequence.len() {
        if sequence[i] == symbol {
            return Some(i);
        }
    }
    None
}

fn get_combi_bf(sequence: &mut Vec<char>, verify: &Vec<u32>, next_question: Option<usize>) -> u32 {
    if let Some(next_question) = next_question {
        // Try with #
        let n = get_next_symbol('?', next_question + 1, sequence);
        sequence[next_question] = '#';
        let hash_res = get_combi_bf(sequence, verify, n);

        // Try with .
        sequence[next_question] = '.';
        let dot_res = get_combi_bf(sequence, verify, n);
        sequence[next_question] = '?';

        hash_res + dot_res
    } else {
        if verify_sequence(sequence, verify) {
            1
        } else {
            0
        }
    }
}

fn get_combinations_brute_force(input: &str) -> u32 {
    let mut s = input.split(" ");
    let fst = s.next().unwrap();
    let snd = s.next().unwrap();

    let mut sequence = fst.chars().chain(iter::once('.')).collect::<Vec<char>>();
    let verify = snd
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let first_question_mark = get_next_symbol('?', 0, &sequence);
    get_combi_bf(&mut sequence, &verify, first_question_mark)
}

fn get_combinations_brute_force_blown_up(input: &&str) -> u32 {
    let mut s = input.split(" ");
    let fst = s.next().unwrap();
    let snd = s.next().unwrap();
    let fst = iter::repeat(fst).take(5).collect::<Vec<&str>>().join("?");
    let snd = iter::repeat(snd).take(5).collect::<Vec<&str>>().join(",");

    let mut sequence = fst.chars().chain(iter::once('.')).collect::<Vec<char>>();
    let verify = snd
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let first_question_mark = get_next_symbol('?', 0, &sequence);
    get_combi_bf(&mut sequence, &verify, first_question_mark)
}

fn get_combinations_verify_wise_sub<'a>(
    cache: &mut HashMap<(&'a [u32], usize), u64>,
    sequence: &Vec<char>,
    verify: &'a [u32],
    from_index: usize,
) -> u64 {
    // Get verify, go through all ? and try and fit a sequence of that length
    // call this function again and again
    if verify.len() == 0 {
        // check everything after from_index is '.' or '?'
        if sequence[from_index..].iter().all(|x| x != &'#') {
            1
        } else {
            0
        }
    } else {
        let key = (verify, from_index);
        if let Some(res) = cache.get(&key) {
            *res
        } else {
            let number_of_springs = verify[0];
            let next_hash =
                get_next_symbol('#', from_index, sequence).unwrap_or(sequence.len() - 2) + 2;
            let res = (from_index..next_hash)
                .map(|i| {
                    if sequence[i - 1] != '#'
                        && (i + number_of_springs as usize) < sequence.len()
                        && sequence[i..(i + number_of_springs as usize)]
                            .iter()
                            .all(|x| x != &'.')
                        && sequence[i + number_of_springs as usize] != '#'
                    {
                        get_combinations_verify_wise_sub(
                            cache,
                            sequence,
                            &verify[1..],
                            i + number_of_springs as usize + 1,
                        )
                    } else {
                        0
                    }
                })
                .sum::<u64>();
            cache.insert(key, res);
            res
        }
    }
}

fn get_combinations_verify_wise_sub_with_output(
    sequence: &Vec<char>,
    verify: &[u32],
    from_index: usize,
) -> Vec<Vec<char>> {
    // Get verify, go through all ? and try and fit a sequence of that length
    // call this function again and again
    if verify.len() == 0 {
        // check everything after from_index is '.' or '?'
        if sequence[from_index..]
            .iter()
            .all(|x| x == &'?' || x == &'.')
        {
            vec![sequence
                .iter()
                .map(|x| if *x == '?' { '.' } else { *x })
                .collect::<Vec<char>>()]
        } else {
            vec![]
        }
    } else {
        let number_of_springs = verify[0];
        let next_hash =
            get_next_symbol('#', from_index, sequence).unwrap_or(sequence.len() - 2) + 2;
        (from_index..next_hash)
            .map(|i| {
                if sequence[i - 1] != '#'
                    && (i + number_of_springs as usize) < sequence.len()
                    && sequence[i..(i + number_of_springs as usize)]
                        .iter()
                        .all(|x| x != &'.')
                {
                    // Possible location
                    match sequence[i + number_of_springs as usize] {
                        '?' | '.' => {
                            let s = get_combinations_verify_wise_sub_with_output(
                                sequence,
                                &verify[1..],
                                i + number_of_springs as usize + 1,
                            );
                            let s = s
                                .into_iter()
                                .map(|mut x| {
                                    for j in i..(i + number_of_springs as usize) {
                                        x[j] = '#';
                                    }
                                    x
                                })
                                .collect::<Vec<Vec<char>>>();
                            s
                        } // Can use this
                        '#' => vec![], // Sequence would be too long
                        _ => panic!("Invalid character"),
                    }
                } else {
                    vec![]
                }
            })
            .fold(vec![], |v, j| [v, j].concat())
    }
}

fn get_combinations_verify_wise(input: &str) -> u64 {
    let mut s = input.split(" ");
    let fst = s.next().unwrap();
    let snd = s.next().unwrap();

    let sequence = iter::once('.')
        .chain(fst.chars().chain(iter::once('.')))
        .collect::<Vec<char>>();
    let verify = snd
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut cache = HashMap::new();
    get_combinations_verify_wise_sub(&mut cache, &sequence, &verify, 1)
}

fn get_combinations_verify_wise_blown_up(input: &&str) -> u64 {
    let mut s = input.split(" ");
    let fst = s.next().unwrap();
    let snd = s.next().unwrap();
    let fst = iter::repeat(fst).take(5).collect::<Vec<&str>>().join("?");
    let snd = iter::repeat(snd).take(5).collect::<Vec<&str>>().join(",");

    let sequence = iter::once('.')
        .chain(fst.chars().chain(iter::once('.')))
        .collect::<Vec<char>>();
    let verify = snd
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut cache = HashMap::new();
    get_combinations_verify_wise_sub(&mut cache, &sequence, &verify, 1)
}

pub struct Task12;

impl TaskCompleter for Task12 {
    fn get_name(&self) -> String {
        "12".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_12/input");
        contents
            .lines()
            .map(get_combinations_verify_wise)
            .sum::<u64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents = include_str!("../input/day_12/input");
        let pb = indicatif::ProgressBar::new(1000);
        zip(0.., contents.lines())
            .collect::<Vec<(i32, &str)>>()
            .par_iter()
            .map(|(i, x)| {
                let start: Instant = Instant::now();
                let v = get_combinations_verify_wise_blown_up(x);
                if start.elapsed() > Duration::from_secs(20) {
                    println!("Line {} took {:?}", i + 1, start.elapsed());
                }
                pb.inc(1);
                v
            })
            .sum::<u64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("7939".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("850504257483930".to_owned())
    }
}
