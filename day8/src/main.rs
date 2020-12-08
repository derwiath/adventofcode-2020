#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[allow(dead_code)]
impl Instruction {
    fn parse(s: &str) -> Option<Instruction> {
        lazy_static! {
            static ref INSTRUCTION_RE: regex::Regex =
                regex::Regex::new(r"([a-z]+) ([+-]\d*)").unwrap();
        }

        println!("line: {}", s);

        if let Some(captures) = INSTRUCTION_RE.captures(s) {
            if captures.len() != 3 {
                return None;
            }

            let name = captures.get(1).unwrap().as_str();
            let arg = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            match name {
                "nop" => Some(Self::Nop(arg)),
                "acc" => Some(Self::Acc(arg)),
                "jmp" => Some(Self::Jmp(arg)),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn solve_part1(input: &str) -> Option<i64> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::parse(line))
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .collect();
    let mut visited: Vec<bool> = Vec::with_capacity(instructions.len());
    for _ in instructions.iter() {
        visited.push(false);
    }
    let mut accumulator: i64 = 0;
    let mut pos: isize = 0;
    loop {
        if pos < 0 {
            break;
        }
        let upos = pos as usize;

        match visited.get(upos) {
            None => break,
            Some(visited) => {
                if *visited {
                    break;
                }
            }
        }
        visited[upos] = true;
        println!("{:?}", instructions[upos]);
        if let Some(instruction) = instructions.get(upos) {
            match instruction {
                Instruction::Nop(_) => {
                    pos += 1;
                }
                Instruction::Acc(arg) => {
                    accumulator += *arg as i64;
                    pos += 1;
                }
                Instruction::Jmp(arg) => {
                    pos += *arg as isize;
                }
            }
        }
    }

    Some(accumulator)
}

fn solve_part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day8 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {:?}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {:?}", answer2);
}

#[cfg(test)]
mod tests8 {
    use super::*;

    const EXAMPLE1: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    #[test]
    fn test1_1() {
        let instructions: [Instruction; 9] = [
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];
        let mut instruction_it = instructions.iter();
        for line in EXAMPLE1.lines().skip(1) {
            let instruction = instruction_it.next();
            assert_eq!(Instruction::parse(line).as_ref(), instruction);
        }
    }

    #[test]
    fn test1_2() {
        assert_eq!(solve_part1(EXAMPLE1), Some(5));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
