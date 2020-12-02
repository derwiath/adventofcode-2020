#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp;
use std::env;
use std::fs;

fn is_password_valid1(line: &str) -> Option<bool> {
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

fn is_password_valid2(line: &str) -> Option<bool> {
    lazy_static! {
        static ref PARSE_RE: regex::Regex =
            regex::Regex::new(r"(\d*)-(\d*) ([a-z]): (.*)").unwrap();
            //regex::Regex::new(r"\(\d*\)-\(\d*\) \([a-z]\): \(.*\)").unwrap();
    }

    if line.len() == 0 {
        None
    } else {
        let captures = PARSE_RE.captures(line).unwrap();
        let pos1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let pos2 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let needle = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = captures.get(4).unwrap().as_str();
        if cmp::max(pos1, pos2) < password.len() {
            Some(
                (password.chars().nth(pos1).unwrap() == needle)
                    ^ (password.chars().nth(pos2).unwrap() == needle),
            )
        } else {
            Some(false)
        }
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

    let invalid_count = count_valid_passwords(&input, is_password_valid1);
    println!("Invalid passwords (Policy 1): {}", invalid_count);

    let invalid_count = count_valid_passwords(&input, is_password_valid2);
    println!("Invalid passwords (Policy 2): {}", invalid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        const EXAMPLE: &str = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(count_valid_passwords(EXAMPLE, is_password_valid1), 2);
        assert_eq!(count_valid_passwords(EXAMPLE, is_password_valid2), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(is_password_valid1(""), None);
    }

    #[test]
    fn test_valid() {
        let line = "8-9 n: nnnnnnnnn";
        assert_eq!(is_password_valid1(line), Some(true));
    }

    #[test]
    fn test_invalid() {
        let line = "8-9 n: nnnnn";
        assert_eq!(is_password_valid1(line), Some(false));
    }
}
