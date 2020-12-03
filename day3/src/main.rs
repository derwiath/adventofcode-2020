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
    let width = map
        .lines()
        .find(|l| {
            if let Some(c) = l.chars().nth(0) {
                c == '.' || c == '#'
            } else {
                false
            }
        })
        .unwrap()
        .chars()
        .filter(|c| *c == '.' || *c == '#')
        .count();

    let stride = slope.x + width * slope.y;
    let mut pos = 0;
    let mut trees = 0;
    let clean_map = map.chars().filter(|c| c == &'.' || c == &'#');
    let map_len = clean_map.clone().count();
    let is_tree = |pos| {
        if let Some(c) = clean_map.clone().nth(pos) {
            c == '#'
        } else {
            false
        }
    };
    while pos < map_len {
        pos += stride;
        if pos < map_len {
            trees += if is_tree(pos) { 1 } else { 0 };
        } else {
            pos -= width;
            trees += if is_tree(pos) { 1 } else { 0 };
            break;
        };
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

    let mut product: u32 = 1;
    for slope in [
        Pos::new(1, 1),
        Pos::new(3, 1),
        Pos::new(5, 1),
        Pos::new(7, 1),
        Pos::new(1, 2),
    ]
    .iter()
    {
        let trees = count_trees(&input, &slope);
        product *= trees;
        println!("Trees {}", trees);
    }
    println!("Product {}", product);
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
    fn test_example1_1() {
        assert_eq!(count_trees(EXAMPLE, &Pos::new(1, 1)), 2);
    }

    #[test]
    fn test_example3_1() {
        assert_eq!(count_trees(EXAMPLE, &Pos::new(3, 1)), 7);
    }

    #[test]
    fn test_example5_1() {
        assert_eq!(count_trees(EXAMPLE, &Pos::new(5, 1)), 3);
    }

    #[test]
    fn test_example7_1() {
        assert_eq!(count_trees(EXAMPLE, &Pos::new(7, 1)), 4);
    }

    #[test]
    fn test_example1_2() {
        assert_eq!(count_trees(EXAMPLE, &Pos::new(1, 2)), 2);
    }
}
