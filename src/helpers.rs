use std::{fs, path::Path};

pub fn get_contents(problem_number: String) -> String {
    let path = format!("./input/{}/input", problem_number);
    fs::read_to_string(path).expect("Should have been able to read the file")
}
