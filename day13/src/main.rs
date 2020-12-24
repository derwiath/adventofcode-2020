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

#[derive(Debug, PartialEq, PartialOrd)]
struct Hit {
    start: u64,
    period: u64,
}

fn first_hit(a: u64, start: u64, b: u64, offset: u64) -> Hit {
    let mut x = start;
    loop {
        if x >= offset && (x + offset) % b == 0 {
            return Hit {
                start: x,
                period: a * b,
            };
        }
        x += a;
    }
}

fn solve_part2(input: &str) -> Option<u64> {
    // 17,x,13,19 => is 3417
    let first = input.split(",").nth(0).unwrap().parse::<u64>().unwrap();
    let bus_ids_and_offsets: Vec<(u64, u64)> = input
        .split(",")
        .enumerate()
        .skip(1)
        .filter(|(_, s)| s != &"x")
        .map(|(i, s)| (s.parse::<u64>().unwrap(), i as u64))
        .collect();
    let mut a = first;
    let mut start = 0;
    for (bus_id, offset) in bus_ids_and_offsets {
        let hit = first_hit(a, start, bus_id, offset);
        println!("{:?}", hit);
        start = hit.start;
        a = hit.period;
    }

    Some(start)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day13 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {:?}", answer1);

    let answer2 = solve_part2(&input.lines().nth(1).unwrap());
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
        assert_eq!(solve_part2(EXAMPLE2_0), Some(1068781));
    }

    #[test]
    fn test2_1() {
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

    #[test]
    fn test2_6() {
        assert_eq!(
            first_hit(17, 0, 13, 2),
            Hit {
                start: 102,
                period: 17 * 13
            }
        );
    }
}
