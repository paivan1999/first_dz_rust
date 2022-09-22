#![forbid(unsafe_code)]

use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut file1_strings = HashSet::new();
    let mut final_strings = HashSet::new();
    let file1 = File::open(&args[0]).unwrap();
    let file2 = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file1);
    for line in reader.lines() {
        file1_strings.insert(line.unwrap());
    }
    let reader = BufReader::new(file2);
    for line in reader.lines() {
        let str_from_line = line.unwrap();
        if file1_strings.contains(&str_from_line) {
            if !final_strings.contains(&str_from_line) {
                println!("{}", str_from_line);
                final_strings.insert(str_from_line);
            }
        }
    }
}
