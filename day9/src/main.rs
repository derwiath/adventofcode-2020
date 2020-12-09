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

fn find_conti<'a>(numbers: &'a [u64], needle: u64) -> Option<&'a [u64]> {
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum == needle {
                return Some(&numbers[i..j + 1]);
            } else if sum > needle {
                break;
            }
        }
    }
    None
}

fn solve_part2(input: &str, preamble: usize) -> Option<u64> {
    let numbers = parse_numbers(input);
    let answer1 = find_first_invalid(&numbers[..], preamble).unwrap();
    match find_conti(&numbers[..], answer1) {
        Some(conti) => {
            let min = conti.iter().min().unwrap();
            let max = conti.iter().max().unwrap();
            Some(min + max)
        }
        None => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: dayx input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input, 25);
    println!("Answer 1: {:?}", answer1);

    let answer2 = solve_part2(&input, 25);
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

    #[test]
    fn test2_1() {
        let numbers = parse_numbers(EXAMPLE1);
        assert_eq!(find_conti(&numbers[..], 127), Some(&numbers[2..6]));
    }

    #[test]
    fn test2_2() {
        assert_eq!(solve_part2(EXAMPLE1, 5), Some(62));
    }
}
