#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_bag(s: &str) -> Option<String> {
    lazy_static! {
        static ref BAG_RE: regex::Regex = regex::Regex::new(r"([a-z]*) ([a-z]*) bag").unwrap();
    }

    match BAG_RE.captures(s) {
        Some(captures) => {
            let variant = captures.get(1).unwrap().as_str();
            let color = captures.get(2).unwrap().as_str();
            Some(format!("{} {}", variant, color))
        }
        None => None,
    }
}

fn parse_bag_with_count(s: &str) -> Option<(String, usize)> {
    lazy_static! {
        static ref BAG_WITH_COUNT_RE: regex::Regex =
            regex::Regex::new(r"([0-9]+) ([a-z]*) ([a-z]*) bag").unwrap();
    }

    match BAG_WITH_COUNT_RE.captures(s) {
        Some(captures) => {
            let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let variant = captures.get(2).unwrap().as_str();
            let color = captures.get(3).unwrap().as_str();
            Some((format!("{} {}", variant, color), count))
        }
        None => None,
    }
}

fn create_parent_bags_map(rules: &str) -> HashMap<String, Vec<String>> {
    let mut parent_bags = HashMap::<String, Vec<String>>::new();
    for rule in rules.lines() {
        if rule.len() == 0 {
            continue;
        }
        let parts: Vec<&str> = rule.split(" contain ").collect();
        let container_bag = parse_bag(parts.get(0).unwrap()).unwrap();
        let contents = parts.get(1).unwrap();
        for contained in contents.split(", ") {
            if let Some((contained_bag, _count)) = parse_bag_with_count(contained) {
                if let Some(parents) = parent_bags.get_mut(&contained_bag) {
                    parents.push(container_bag.clone());
                } else {
                    let mut parents = Vec::<String>::new();
                    parents.push(container_bag.clone());
                    parent_bags.insert(contained_bag.clone(), parents);
                }
            }
        }
    }
    parent_bags
}

fn create_child_bags_map(rules: &str) -> HashMap<String, Vec<(String, usize)>> {
    let mut children_bags = HashMap::<String, Vec<(String, usize)>>::new();
    for rule in rules.lines() {
        if rule.len() == 0 {
            continue;
        }
        let parts: Vec<&str> = rule.split(" contain ").collect();
        let container_bag = parse_bag(parts.get(0).unwrap()).unwrap();
        let contents = parts.get(1).unwrap();
        for contained in contents.split(", ") {
            if let Some((contained_bag, count)) = parse_bag_with_count(contained) {
                if let Some(children) = children_bags.get_mut(&container_bag) {
                    children.push((contained_bag, count));
                } else {
                    let mut children = Vec::<(String, usize)>::new();
                    children.push((contained_bag.clone(), count));
                    children_bags.insert(container_bag.clone(), children);
                }
            }
        }
    }
    children_bags
}

fn solve_part1(rules: &str, bag: &str) -> usize {
    let parent_bags = create_parent_bags_map(rules);

    let mut visited = HashSet::<&str>::new();
    let mut to_process = Vec::<&str>::new();
    let mut i = 0;
    to_process.push(bag);
    while i < to_process.len() {
        let b: &str = to_process.get(i).unwrap();
        let parents: Option<&Vec<String>> = parent_bags.get(b);
        i += 1;
        if let Some(p) = parents {
            for parent in p {
                if !visited.contains(&parent[..]) {
                    to_process.push(&parent);
                    visited.insert(&parent);
                }
            }
        }
    }

    to_process.len() - 1
}

fn solve_part2(rules: &str, bag: &str) -> usize {
    let children_bags = create_child_bags_map(rules);

    let mut queue = Vec::<(&str, usize)>::new();
    let mut i = 0;
    let mut contained_bag_count = 0;
    queue.push((bag, 1));
    while i < queue.len() {
        let (b, multiplier) = match queue.get(i) {
            Some((b, m)) => (b.to_string(), m.clone()),
            None => panic!("gaah"),
        };
        let children: Option<&Vec<(String, usize)>> = children_bags.get(&b[..]);
        i += 1;
        if let Some(children) = children {
            for (child, count) in children.iter() {
                contained_bag_count += multiplier * count;
                queue.push((child.as_str(), multiplier * count));
            }
        }
    }
    contained_bag_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day6 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input, "shiny gold");
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input, "shiny gold");
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests7 {
    use super::*;

    const EXAMPLE: &str = r"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    const EXAMPLE2: &str = r"
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn test_1() {
        assert_eq!(solve_part1(EXAMPLE, "shiny gold"), 4);
    }

    #[test]
    fn test_2_1() {
        assert_eq!(solve_part2(EXAMPLE, "shiny gold"), 32);
    }

    #[test]
    fn test_2_2() {
        assert_eq!(solve_part2(EXAMPLE2, "shiny gold"), 126);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            parse_bag_with_count("2 dark violet bags."),
            Some(("dark violet".to_string(), 2))
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            parse_bag_with_count("1 dark violet bag"),
            Some(("dark violet".to_string(), 1))
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(parse_bag_with_count("no other bags"), None);
    }
}
