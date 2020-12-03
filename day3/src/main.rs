#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day1 input-filename");
    lazy_static! {
        static ref PARSE_RE: regex::Regex =
            regex::Regex::new(r"(\d*)-(\d*) ([a-z]): (.*)").unwrap();
            //regex::Regex::new(r"\(\d*\)-\(\d*\) \([a-z]\): \(.*\)").unwrap();
    }

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    for line in input.lines() {
        println!("{}, {}", line, PARSE_RE.is_match(line));
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn test_example() {
        const EXAMPLE: &str = r"";
        assert_eq!(EXAMPLE.len(), 0);
    }
}
