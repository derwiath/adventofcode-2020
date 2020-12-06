//#[macro_use]
//extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;
use std::env;
use std::fs;

fn solve_part1(input: &str) -> usize {
    /*
    lazy_static! {
        static ref ANSWER_RE: regex::Regex = regex::Regex::new(r"([a-z]*)").unwrap();
    }*/
    let mut set = HashSet::<char>::new();
    let mut sum: usize = 0;
    for line in input.lines() {
        for c in line.chars() {
            if c >= 'a' && c <= 'z' {
                set.insert(c);
            }
        }
        if line.len() == 0 {
            sum += set.len();
            set.clear();
        }
    }
    sum + set.len()
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
mod tests6 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_part1("abc"), 3);
        assert_eq!(solve_part1("a\na\na\n"), 1);
        assert_eq!(solve_part1("a\nb\nc\n"), 3);
        assert_eq!(solve_part1("ab\nac"), 3);
        assert_eq!(solve_part1("a\na\na\na\n"), 1);
        assert_eq!(solve_part1("b"), 1);
    }

    #[test]
    fn test_2() {
        let input = r"
abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve_part1(input), 11);
    }
}
