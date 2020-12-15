#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fmt;
use std::fs;
use std::ops;

/*
 *      ^ +N
 *      |
 *      |
 * W ---|---> +E
 *      |
 *      |
 *      S
 */

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Move(Vec2),
    Turn(u64),
    Forward(i64),
}

#[allow(dead_code)]
impl Instruction {
    pub fn parse(s: &str) -> Option<Instruction> {
        lazy_static! {
            static ref INSTRUCTION_RE: regex::Regex =
                regex::Regex::new(r"([NSEWLRF])([+-]*\d*)").unwrap();
        }

        if let Some(captures) = INSTRUCTION_RE.captures(s) {
            if captures.len() != 3 {
                return None;
            }

            let name = captures.get(1).unwrap().as_str();
            let arg = captures.get(2).unwrap().as_str().parse::<u64>().unwrap() as i64;
            match name {
                "N" => Some(Self::Move(Vec2::new(0, arg))),
                "S" => Some(Self::Move(Vec2::new(0, -arg))),
                "E" => Some(Self::Move(Vec2::new(arg, 0))),
                "W" => Some(Self::Move(Vec2::new(-arg, 0))),
                "L" | "R" => {
                    assert_eq!(arg % 90, 0);
                    let turns = arg / 90;
                    let turns = turns % 4;
                    let turns = if name == "R" { turns } else { 4 - turns };
                    Some(Self::Turn(turns as u64))
                }
                "F" => Some(Self::Forward(arg)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Move(trans) => write!(f, "Move {}", trans),
            Instruction::Turn(turns) => write!(f, "Turn {}", turns),
            Instruction::Forward(dist) => write!(f, "Forward {}", dist),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(&self, turns: &u64) -> Direction {
        const TURN_ORDER: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let current = TURN_ORDER.iter().position(|d| d == self).unwrap();
        let next = (current + *turns as usize) % TURN_ORDER.len();
        TURN_ORDER.get(next).unwrap().clone()
    }

    fn to_vec2(&self) -> Vec2 {
        match self {
            Self::North => Vec2::new(0, 1),
            Self::South => Vec2::new(0, -1),
            Self::East => Vec2::new(1, 0),
            Self::West => Vec2::new(-1, 0),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Direction::North => "N",
            Direction::South => "S",
            Direction::East => "E",
            Direction::West => "W",
        };
        write!(f, "{}", name)
    }
}

#[allow(dead_code)]
struct Ship {
    pos: Vec2,
    dir: Direction,
}

#[allow(dead_code)]
impl Ship {
    fn new() -> Self {
        Self {
            pos: Vec2::new(0, 0),
            dir: Direction::East,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(trans) => {
                self.pos += trans.clone();
            }
            Instruction::Turn(turns) => {
                self.dir = self.dir.turn_right(turns);
            }
            Instruction::Forward(dist) => {
                let trans = self.dir.to_vec2() * *dist;
                self.pos += trans;
            }
        };
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.pos, self.dir)
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, usize> {
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.len() > 0)
        .map(|(line_index, line)| {
            println!("{}: {}", line_index + 1, line);
            if let Some(instruction) = Instruction::parse(line) {
                Ok(instruction)
            } else {
                Err(line_index)
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let instructions = parse_instructions(input).unwrap();
    let mut ship = Ship::new();
    instructions.iter().for_each(|i| {
        print!("{} + {} = ", ship, i);
        ship.apply(i);
        println!("{}", ship);
    });
    ship.pos.manhattan_distance() as usize
}

fn solve_part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day12 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests12 {
    use super::*;

    const EXAMPLE1: &str = "\
            F10\n\
            N3\n\
            F7\n\
            R90\n\
            F11";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 25);
    }

    #[test]
    fn test1_2() {
        assert_eq!(Instruction::parse("F10"), Some(Instruction::Forward(10)));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
