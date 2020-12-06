//#[macro_use]
//extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn solve_part1(input: &str) -> usize {
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
    let mut map = HashMap::<char, usize>::new();
    let mut sum: usize = 0;
    let mut group_size = 0;
    for line in input.lines() {
        if line.len() > 0 {
            group_size += 1;
            for c in line.chars() {
                if c >= 'a' && c <= 'z' {
                    if let Some(count) = map.get_mut(&c) {
                        *count += 1;
                    } else {
                        map.insert(c, 1);
                    }
                }
            }
        } else {
            sum += map.iter().filter(|(_, v)| *v >= &group_size).count();
            map.clear();
            group_size = 0;
        }
    }
    sum + map.iter().filter(|(_, v)| *v >= &group_size).count()
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

    #[test]
    fn test_3() {
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
        assert_eq!(solve_part2(input), 6);
    }
}
