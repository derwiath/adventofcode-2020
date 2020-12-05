#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp;
use std::env;
use std::fs;

fn calc_seat_id(s: &str) -> usize {
    let mut id: usize = 0;
    for c in s.chars() {
        id = id << 1;
        if c == 'B' || c == 'R' {
            id = id | 0x1
        }
    }
    id
}

fn main() {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"hello").unwrap();
    }
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day4 input-filename");
    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");
    let mut max_id = 0;
    for line in input.lines() {
        let seat_id = calc_seat_id(line);
        max_id = cmp::max(seat_id, max_id);
    }

    println!("Max seat id {}", max_id);
}

#[cfg(test)]
mod tests5 {
    use super::*;

    const EXAMPLES: [(&str, usize); 4] = [
        ("FBFBBFFRLR", 357),
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ];

    #[test]
    fn test_1() {
        for (example, seat_id) in EXAMPLES.iter() {
            assert_eq!(calc_seat_id(example), *seat_id);
        }
    }
}
