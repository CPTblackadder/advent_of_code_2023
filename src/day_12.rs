use crate::TaskCompleter;
use rayon::prelude::*;
use std::iter;

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
                    if verify_index >= verify.len() && x != verify[verify_index] {
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

fn get_next_question_mark(from: usize, sequence: &Vec<char>) -> Option<usize> {
    for i in from..sequence.len() {
        match sequence[i] {
            '?' => return Some(i),
            _ => (),
        }
    }
    None
}

fn get_combi_bf(sequence: &mut Vec<char>, verify: &Vec<u32>, next_question: Option<usize>) -> u32 {
    if let Some(next_question) = next_question {
        // Try with #
        let n = get_next_question_mark(next_question + 1, sequence);
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

    let first_question_mark = get_next_question_mark(0, &sequence);
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

    let first_question_mark = get_next_question_mark(0, &sequence);
    get_combi_bf(&mut sequence, &verify, first_question_mark)
}

pub struct Task12;

impl TaskCompleter for Task12 {
    fn get_name(&self) -> String {
        "12".to_owned()
    }

    fn do_task_1(&self) -> String {
        let contents = include_str!("../input/day_12/input");
        contents
            .lines()
            .map(get_combinations_brute_force)
            .sum::<u32>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents = include_str!("../input/day_12/example");
        contents
            .lines()
            .collect::<Vec<&str>>()
            .par_iter()
            .map(get_combinations_brute_force_blown_up)
            .sum::<u32>()
            .to_string()
    }
}
