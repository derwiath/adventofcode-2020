#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

fn solve_part1(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d*) ([a-z]*)").unwrap();
    }
    let mut sum = 0;
    for line in input.lines() {
        if let Some(captures) = RE.captures(line) {
            if captures.len() == 3 {
                let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let _thing = captures.get(2).unwrap().as_str();
                sum += count;
            }
        }
    }
    sum
}

fn solve_part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const PACKAGE_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    let filename = args
        .get(1)
        .expect(format!("Usage: {} input-filename", PACKAGE_NAME.unwrap()).as_str());

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod testsx {
    use super::*;

    const EXAMPLE1: &str = "
3 seals
4 quacks";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 7);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
