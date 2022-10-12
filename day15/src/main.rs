use std::collections::HashMap;
use std::env;
use std::fs;

struct Info {
    last_turn_seen: usize,
    next_last_turn_seen: usize,
}

impl Info {
    fn new(last_turn_seen: usize, next_last_turn_seen: usize) -> Info {
        Info {
            last_turn_seen,
            next_last_turn_seen,
        }
    }
}

fn solve_part1(input: &str, turns: usize) -> usize {
    let input_numbers = input.split(",");
    let mut number_to_turn = HashMap::new();
    let mut turn: usize = 1;
    let mut last_number = 0;
    for input_number in input_numbers {
        //println!("({})", input_number);
        let number = input_number.trim_end().parse::<usize>().unwrap();
        number_to_turn.insert(number, Info::new(turn, 0));
        //println!("{}: {} -> ({}, {})", turn, number, turn, 0);
        last_number = number;
        turn += 1;
    }

    while turn <= turns {
        let number = if let Some(last_info) = number_to_turn.get(&last_number) {
            /*println!(
                "{}: {} -> ({}, {}) (seen)",
                turn, last_number, last_info.last_turn_seen, last_info.next_last_turn_seen
            );*/
            if last_info.next_last_turn_seen > 0 {
                last_info.last_turn_seen - last_info.next_last_turn_seen
            } else {
                0
            }
        } else {
            /*println!("{}: {} -> ({}, {}) (not seen)", turn, 0, turn, 0);*/
            0
        };
        let next_last_turn_seen = if let Some(info) = number_to_turn.get(&number) {
            info.last_turn_seen
        } else {
            0
        };
        let info = Info::new(turn, next_last_turn_seen);
        /*println!(
            "{}: {} -> ({}, {})",
            turn, number, turn, next_last_turn_seen
        );*/
        number_to_turn.insert(number, info);
        last_number = number;
        turn += 1;
    }

    return last_number;
}

#[allow(dead_code)]
fn solve_part2(input: &str, turns: usize) -> usize {
    solve_part1(input, turns)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const PACKAGE_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    let filename = args
        .get(1)
        .expect(format!("Usage: {} input-filename", PACKAGE_NAME.unwrap()).as_str());

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input, 2020);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input, 30000000);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests15 {
    use super::*;

    #[test]
    fn test1_example_10() {
        assert_eq!(solve_part1("0,3,6", 10), 0);
    }

    #[test]
    fn test1_example_4() {
        assert_eq!(solve_part1("0,3,6", 4), 0);
    }
    #[test]
    fn test1_example_5() {
        assert_eq!(solve_part1("0,3,6", 5), 3);
    }

    #[test]
    fn test1_0() {
        assert_eq!(solve_part1("0,3,6", 2020), 436);
    }

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1("1,3,2", 2020), 1);
    }

    #[test]
    fn test1_2() {
        assert_eq!(solve_part1("2,1,3", 2020), 10);
    }

    #[test]
    fn test1_3() {
        assert_eq!(solve_part1("1,2,3", 2020), 27);
    }

    #[test]
    fn test1_4() {
        assert_eq!(solve_part1("2,3,1", 2020), 78);
    }

    #[test]
    fn test1_5() {
        assert_eq!(solve_part1("3,2,1", 2020), 438);
    }

    #[test]
    fn test1_6() {
        assert_eq!(solve_part1("3,1,2", 2020), 1836);
    }
    #[test]
    fn test2_1() {
        assert_eq!(solve_part2("0,3,6", 30000000), 175594);
    }

    #[test]
    fn test2_2() {
        assert_eq!(solve_part2("1,3,2", 30000000), 2578);
    }

    #[test]
    fn test2_3() {
        assert_eq!(solve_part2("2,1,3", 30000000), 3544142);
    }

    #[test]
    fn test2_4() {
        assert_eq!(solve_part2("1,2,3", 30000000), 261214);
    }

    #[test]
    fn test2_5() {
        assert_eq!(solve_part2("2,3,1", 30000000), 6895259);
    }

    #[test]
    fn test2_6() {
        assert_eq!(solve_part2("3,2,1", 30000000), 18);
    }

    #[test]
    fn test2_7() {
        assert_eq!(solve_part2("3,1,2", 30000000), 362);
    }
}
