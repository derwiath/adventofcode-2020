use std::env;
use std::fs;

fn parse_adapters(input: &str) -> Vec<u64> {
    let mut adapters: Vec<u64> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    adapters.sort_unstable();
    let max_adapter = adapters[adapters.len() - 1];
    adapters.push(max_adapter + 3);
    adapters
}
/*
1
4
5
6
7
10
11
12
15
16
19
*/
fn solve_part1(input: &str) -> u64 {
    let adapters = parse_adapters(input);
    let mut jolt_counts: [u64; 3] = [0, 0, 0];
    let mut outlet: u64 = 0;
    for adapter in adapters {
        let diff = adapter - outlet;
        if diff > 3 {
            panic!("adapter {} out of range, outlet {}", adapter, outlet);
        }
        jolt_counts[(diff - 1) as usize] += 1;
        println!(
            "adapter {:3}, outlet {:3}, diff {:1}, jolt_counts {:3} {:3}",
            adapter, outlet, diff, jolt_counts[0], jolt_counts[2]
        );
        outlet = adapter;
    }

    jolt_counts[0] * jolt_counts[2]
}

fn solve_part2(_input: &str) -> u64 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: dayx input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests10 {
    use super::*;

    const EXAMPLE1: &str = "16
10
15
5
1
11
7
19
6
12
4
";
    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 7 * 5);
    }

    const EXAMPLE2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn test1_2() {
        assert_eq!(solve_part1(EXAMPLE2), 22 * 10);
    }

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
