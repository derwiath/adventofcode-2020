#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_bag(s: &str) -> Option<String> {
    lazy_static! {
        static ref BAG_RE: regex::Regex =
            //regex::Regex::new(r"[0-9] ([a-z]*) ([a-z]*) bag[s\.]*").unwrap();
            regex::Regex::new(r"([a-z]*) ([a-z]*) bag").unwrap();
    }

    match BAG_RE.captures(s) {
        Some(captures) => {
            let shade = captures.get(1).unwrap().as_str();
            let color = captures.get(2).unwrap().as_str();
            Some(format!("{} {}", shade, color))
        }
        None => return None,
    }
}

fn solve_part1(rules: &str, bag: &str) -> usize {
    let mut parent_bags = HashMap::<String, Vec<String>>::new();
    for rule in rules.lines() {
        if rule.len() == 0 {
            continue;
        }
        println!("{}", rule);
        let parts: Vec<&str> = rule.split(" contain ").collect();
        let container_bag = parse_bag(parts.get(0).unwrap()).unwrap();
        let contents = parts.get(1).unwrap();
        for content in contents.split(", ") {
            let content_bag = parse_bag(content).unwrap();
            if let Some(parents) = parent_bags.get_mut(&content_bag) {
                parents.push(container_bag.clone());
            } else {
                let mut parents = Vec::<String>::new();
                parents.push(container_bag.clone());
                parent_bags.insert(content_bag, parents);
            }
        }
    }

    for (bag, parents) in parent_bags.iter() {
        println!("{} -> {:?}", bag, parents);
    }

    let mut visited = HashSet::<&str>::new();
    let mut to_process = Vec::<&str>::new();
    let mut i = 0;
    to_process.push(bag);
    while i < to_process.len() {
        let b: &str = to_process.get(i).unwrap();
        let parents: Option<&Vec<String>> = parent_bags.get(b);
        println!("processing {} {:?}", b, parents);
        i += 1;
        if let Some(p) = parents {
            for parent in p {
                if !visited.contains(&parent[..]) {
                    println!("enqueing {}", parent);
                    to_process.push(&parent);
                    visited.insert(&parent);
                }
            }
        }
    }

    to_process.len() - 1
}

fn solve_part2(rules: &str) -> usize {
    rules.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day6 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input, "shiny gold");
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
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

    #[test]
    fn test_1() {
        assert_eq!(solve_part1(EXAMPLE, "shiny gold"), 4);
    }
}
