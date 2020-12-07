#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

fn solve_part1(input: &str) -> usize {
    input.len()
}

fn solve_part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day6 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests7 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}
