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

    #[test]
    fn test2_0() {
        const EXAMPLE2_0: &str = "7,13,x,x,59,x,31,19";
        assert_eq!(solve_part2(EXAMPLE2_0), Some(1068788));
    }

    #[test]
    fn test2_1() {
        /*
        (t + 0) % 17 = 0
        (t + 2) % 13 = 0
        (t + 3) % 19 = 0
        */
        const EXAMPLE2_1: &str = "17,x,13,19"; // is 3417.
        assert_eq!(solve_part2(EXAMPLE2_1), Some(3417));
    }

    #[test]
    fn test2_2() {
        const EXAMPLE2_2: &str = "67,7,59,61"; // first occurs at timestamp 754018.
        assert_eq!(solve_part2(EXAMPLE2_2), Some(754018));
    }

    #[test]
    fn test2_3() {
        const EXAMPLE2_3: &str = "67,x,7,59,61"; // first occurs at timestamp 779210.
        assert_eq!(solve_part2(EXAMPLE2_3), Some(779210));
    }

    #[test]
    fn test2_4() {
        const EXAMPLE2_4: &str = "67,7,x,59,61"; // first occurs at timestamp 1261476.
        assert_eq!(solve_part2(EXAMPLE2_4), Some(1261476));
    }

    #[test]
    fn test2_5() {
        const EXAMPLE2_5: &str = "1789,37,47,1889"; // first occurs at timestamp 1202161486.
        assert_eq!(solve_part2(EXAMPLE2_5), Some(1202161486));
    }
}
