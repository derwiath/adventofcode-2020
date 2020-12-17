use std::env;
use std::fs;

fn solve_part1(input: &str) -> Option<u64> {
    let time = input.lines().nth(0).unwrap().parse::<u64>().unwrap();
    let (bus_id, wait_time) = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .filter(|s| s != &"x")
        .map(|s| s.parse::<u64>().unwrap())
        .map(|bus_id| (bus_id, bus_id - &time % bus_id))
        .min_by_key(|(_, wait_time)| *wait_time)
        .unwrap();
    println!("time {}", time);
    println!("bus_id {}", bus_id);
    println!("wait_time{}", wait_time);
    Some(bus_id * wait_time)
}

fn solve_part2(_input: &str) -> Option<u64> {
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day13 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {:?}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {:?}", answer2);
}

#[cfg(test)]
mod tests13 {
    use super::*;

    const EXAMPLE1: &str = "\
            939\n\
            7,13,x,x,59,x,31,19";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), Some(295));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), None);
    }
}
