use std::env;
use std::fs;

use std::collections::HashMap;

fn parse_adapters(input: &str) -> Vec<usize> {
    let mut adapters: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    adapters.sort_unstable();
    adapters
}

fn solve_part1(input: &str) -> usize {
    let adapters = {
        let mut adapters = parse_adapters(input);
        let max_adapter = adapters[adapters.len() - 1];
        adapters.push(max_adapter + 3);
        adapters
    };
    let mut jolt_counts: [usize; 3] = [0, 0, 0];
    let mut outlet: usize = 0;
    for adapter in adapters {
        let diff = adapter - outlet;
        if diff > 3 {
            panic!("adapter {} out of range, outlet {}", adapter, outlet);
        }
        jolt_counts[diff - 1] += 1;
        outlet = adapter;
    }

    jolt_counts[0] * jolt_counts[2]
}

fn get_combos(next_combos: &[(usize, usize)], cache: &mut HashMap<usize, usize>) -> usize {
    if next_combos.len() == 0 {
        return 1;
    }
    let (adapter, combos) = next_combos[0];
    if let Some(cached) = cache.get(&adapter) {
        return *cached;
    }
    let mut res = 0;
    for i in 1..combos + 1 {
        res += get_combos(&next_combos[i..], cache)
    }
    cache.insert(adapter, res);
    res
}

fn solve_part2(input: &str) -> usize {
    let adapters = {
        let mut adapters = parse_adapters(input);
        let max_adapter = adapters[adapters.len() - 1];
        adapters.insert(0, 0);
        adapters.push(max_adapter + 3);
        adapters
    };
    let next_combos: Vec<(usize, usize)> = adapters
        .iter()
        .enumerate()
        .map(|(i, adapter)| {
            let combos = match adapters[i + 1..]
                .iter()
                .position(|next| next - *adapter > 3)
            {
                Some(pos) => pos,
                None => adapters.len() - (i + 1),
            };
            (*adapter, combos)
        })
        .collect();
    let mut cache = HashMap::<usize, usize>::with_capacity(next_combos.len() * 2);
    get_combos(&next_combos[..next_combos.len() - 1], &mut cache)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day10 input-filename");

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
        assert_eq!(solve_part2(EXAMPLE1), 8);
    }

    #[test]
    fn test2_2() {
        assert_eq!(solve_part2(EXAMPLE2), 19208);
    }

    const EXAMPLE3: &str = "1
4
5
6
7
10
";
    #[test]
    fn test2_3() {
        assert_eq!(solve_part2(EXAMPLE3), 4);
    }
}
