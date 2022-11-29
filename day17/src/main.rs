#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

struct Point(usize, usize, usize);
struct GridSize(usize, usize, usize);

struct Grid<T> {
    values: Vec<T>,
    size: GridSize,
    slice_size: usize,
}

impl<T> Grid<T> {
    fn with_size(size: GridSize) -> Grid<T> {
        let slice_size = size.0 * size.1;
        let values = Vec::with_capacity(slice_size * size.2);
        Grid {
            values,
            size,
            slice_size,
        }
    }

    fn get(&self, p: &Point) -> Option<&T> {
        let index = p.0 + p.1 * self.size.0 + p.2 * self.slice_size;
        self.values.get(index)
    }
}

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
    /*
     * 8x8x1
     * 10x10x3
     * 8 + 2 * 6 = 20
     * 1 + 2 * 6 = 13
     *
     *
     */
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
mod tests17 {
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
