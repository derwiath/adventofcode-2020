#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn count_trees(map: &str, slope: &Pos) -> u32 {
    lazy_static! {
        static ref MAP_RE: regex::Regex = regex::Regex::new(r"\([\.#]*^\)").unwrap();
    }

    let rows: Vec<&str> = map
        .lines()
        .filter(|r| r.len() > 0)
        .map(|r| {
            if r.ends_with("  --->") {
                &r[0..r.len() - "  --->".len()]
            } else {
                &r[..]
            }
        })
        .collect();
    let mut trees = 0;
    let mut pos = Pos::new(0, 0);
    let width = rows.first().unwrap().len();
    while pos.y < rows.len() {
        pos.x = (pos.x + slope.x) % width;
        pos.y += slope.y;
        let tree = if let Some(row) = rows.get(pos.y) {
            if let Some(c) = row.chars().nth(pos.x) {
                c == '#'
            } else {
                false
            }
        } else {
            false
        };
        if tree {
            trees += 1;
        }

        println!("{} {}: tree {}, count {}", pos.x, pos.y, tree, trees);
    }
    trees
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day1 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let slope = Pos::new(3, 1);
    println!("Trees {}", count_trees(&input, &slope));
}

#[cfg(test)]
mod tests3 {
    use super::*;

    const EXAMPLE: &str = r"
..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
";
    #[test]
    fn test_example() {
        let slope = Pos::new(3, 1);
        assert_eq!(count_trees(EXAMPLE, &slope), 7);
    }
}
