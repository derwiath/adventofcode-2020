#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

fn main() {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"hello").unwrap();
    }
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day4 input-filename");
    println!("Reading input from {}", filename);
}

#[cfg(test)]
mod tests5 {
    use super::*;

    const EXAMPLE: &str = "";
    #[test]
    fn test_1() {
        assert_eq!(1 + 1, 2);
    }
}
