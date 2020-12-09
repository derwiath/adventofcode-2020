use std::env;
use std::fs;

fn parse_numbers(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect()
}

fn is_number_valid(number: u64, preamble: &[u64]) -> bool {
    for i in 0..preamble.len() - 1 {
        let p = preamble[i];
        if p > number {
            continue;
        }
        let needle = number - p;
        if needle == p {
            continue;
        }
        if preamble[i + 1..].contains(&needle) {
            return true;
        }
    }
    false
}

fn find_first_invalid(numbers: &[u64], preamble: usize) -> Option<u64> {
    for i in preamble..numbers.len() {
        let number = numbers[i];
        if !is_number_valid(numbers[i], &numbers[i - preamble..i]) {
            return Some(number);
        }
    }
    None
}

fn solve_part1(input: &str, preamble: usize) -> Option<u64> {
    let numbers = parse_numbers(input);
    find_first_invalid(&numbers[..], preamble)
}

fn solve_part2(_input: &str) -> Option<u64> {
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: dayx input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input, 25);
    println!("Answer 1: {:?}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {:?}", answer2);
}

#[cfg(test)]
mod tests9 {
    use super::*;

    const EXAMPLE1: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1, 5), Some(127));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), None);
    }
}
