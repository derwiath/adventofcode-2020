#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

fn solve_part1(input: &str, _: u32) -> usize {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    const PACKAGE_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    let filename = args
        .get(1)
        .expect(format!("Usage: {} input-filename", PACKAGE_NAME.unwrap()).as_str());

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input, 2020);
    println!("Answer 1: {}", answer1);
}

#[cfg(test)]
mod tests15 {
    use super::*;

    #[test]
    fn test1_0() {
        assert_eq!(solve_part1("0,3,6", 2020), 436);
    }

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1("1,3,2", 2020), 1);
    }

    #[test]
    fn test1_2() {
        assert_eq!(solve_part1("2,1,3", 2020), 10);
    }

    #[test]
    fn test1_3() {
        assert_eq!(solve_part1("1,2,3", 2020), 27);
    }

    #[test]
    fn test1_4() {
        assert_eq!(solve_part1("2,3,1", 2020), 78);
    }

    #[test]
    fn test1_5() {
        assert_eq!(solve_part1("3,2,1", 2020), 438);
    }

    #[test]
    fn test1_6() {
        assert_eq!(solve_part1("3,1,2", 2020), 1836);
    }
}
