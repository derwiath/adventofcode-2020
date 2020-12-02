#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

fn is_password_valid(line: &str) -> Option<bool> {
    lazy_static! {
        static ref PARSE_RE: regex::Regex =
            regex::Regex::new(r"(\d*)-(\d*) ([a-z]): (.*)").unwrap();
            //regex::Regex::new(r"\(\d*\)-\(\d*\) \([a-z]\): \(.*\)").unwrap();
    }

    if line.len() == 0 {
        None
    } else {
        let captures = PARSE_RE.captures(line).unwrap();
        let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let needle = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = captures.get(4).unwrap().as_str();
        let count = password.chars().filter(|c| c == &needle).count();
        Some(min <= count && count <= max)
    }
}

fn count_valid_passwords<P>(input: &str, policy: P) -> usize
where
    P: Fn(&str) -> Option<bool>,
{
    input
        .lines()
        .filter(|line| {
            if let Some(valid) = policy(line) {
                valid
            } else {
                false
            }
        })
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day1 input-filename");

    println!("Reading terms from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let invalid_count = count_valid_passwords(&input, is_password_valid);
    println!("Invalid passwords: {}", invalid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        const EXAMPLE: &str = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(count_valid_passwords(EXAMPLE, is_password_valid), 2);
    }
    #[test]
    fn test_empty() {
        assert_eq!(is_password_valid(""), None);
    }

    #[test]
    fn test_valid() {
        let line = "8-9 n: nnnnnnnnn";
        assert_eq!(is_password_valid(line), Some(true));
    }

    #[test]
    fn test_invalid() {
        let line = "8-9 n: nnnnn";
        assert_eq!(is_password_valid(line), Some(false));
    }
}
