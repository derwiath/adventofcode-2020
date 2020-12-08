#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(isize),
}

#[allow(dead_code)]
impl Instruction {
    fn parse(s: &str) -> Option<Instruction> {
        lazy_static! {
            static ref INSTRUCTION_RE: regex::Regex =
                regex::Regex::new(r"([a-z]+) ([+-]\d*)").unwrap();
        }

        if let Some(captures) = INSTRUCTION_RE.captures(s) {
            if captures.len() != 3 {
                return None;
            }

            let name = captures.get(1).unwrap().as_str();
            let arg = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            match name {
                "nop" => Some(Self::Nop(arg)),
                "acc" => Some(Self::Acc(arg)),
                "jmp" => Some(Self::Jmp(arg as isize)),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum ExitError {
    InfiniteLoop(i64, usize),
    InvalidPC(isize, usize),
}

fn parse_program(input: &str) -> Result<Vec<Instruction>, usize> {
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

fn execute_program(program: &Vec<Instruction>) -> Result<i64, ExitError> {
    let mut visited: Vec<bool> = Vec::with_capacity(program.len());
    for _ in 0..program.len() {
        visited.push(false);
    }
    let mut accumulator: i64 = 0;
    let mut pos: usize = 0;
    while pos < program.len() {
        match visited.get(pos) {
            None => break,
            Some(visited) => {
                if *visited {
                    return Err(ExitError::InfiniteLoop(accumulator, pos));
                }
            }
        }
        visited[pos] = true;
        println!("{:?} [{}]", program.get(pos), pos);
        let offset: isize = match program.get(pos) {
            Some(Instruction::Nop(_)) => 1,
            Some(Instruction::Acc(number)) => {
                accumulator += *number;
                1
            }
            Some(Instruction::Jmp(offset)) => *offset,
            None => {
                return Err(ExitError::InvalidPC(pos as isize, pos));
            }
        };
        let next_pos: isize = pos as isize + offset;
        if next_pos < 0 {
            return Err(ExitError::InvalidPC(next_pos, pos));
        } else if next_pos as usize > program.len() {
            return Err(ExitError::InvalidPC(next_pos, pos));
        }
        pos = next_pos as usize;
    }

    Ok(accumulator)
}

fn solve_part1(program: &Vec<Instruction>) -> Option<i64> {
    match execute_program(program) {
        Ok(accumulator) => Some(accumulator),
        Err(ExitError::InvalidPC(_, _)) => None,
        Err(ExitError::InfiniteLoop(accumulator, _)) => Some(accumulator),
    }
}

fn solve_part2(program: &Vec<Instruction>) -> Option<(i64, usize)> {
    let mut next_pos: Option<usize> = program.iter().position(|i| match i {
        Instruction::Nop(_) => true,
        _ => false,
    });
    while next_pos.is_some() {
        let mut copy = program.clone();
        let pos = next_pos.unwrap();
        let arg = match copy.get(pos) {
            Some(Instruction::Nop(arg)) => arg,
            _ => {
                panic!("wtf");
            }
        };
        println!(
            "{}: {:?} -> {:?}",
            pos,
            copy[pos],
            Instruction::Jmp(*arg as isize)
        );
        copy[pos] = Instruction::Jmp(*arg as isize);

        match execute_program(&copy) {
            Ok(accumulator) => return Some((accumulator, pos)),
            _ => (),
        }
        next_pos = program
            .iter()
            .skip(pos + 1)
            .position(|i| match i {
                Instruction::Nop(_) => true,
                _ => false,
            })
            .map(|found_pos| found_pos + pos + 1);
    }

    let mut next_pos = program.iter().position(|instr| match instr {
        Instruction::Jmp(_) => true,
        _ => false,
    });
    println!("{:?}", next_pos);
    while next_pos.is_some() {
        let mut copy = program.clone();
        let pos = next_pos.unwrap();
        let arg = match copy.get(pos) {
            Some(Instruction::Jmp(arg)) => arg,
            Some(x) => {
                panic!(format!("{}: wtf {:?}", pos, x));
            }
            None => {
                panic!("wtf none");
            }
        };
        println!(
            "{}: {:?} -> {:?}",
            pos + 1,
            copy[pos],
            Instruction::Nop(*arg as i64)
        );
        copy[pos] = Instruction::Nop(*arg as i64);

        match execute_program(&copy) {
            Ok(accumulator) => return Some((accumulator, pos)),
            _ => (),
        }
        next_pos = program
            .iter()
            .skip(pos + 1)
            .position(|i| match i {
                Instruction::Jmp(_) => true,
                _ => false,
            })
            .map(|found_pos| found_pos + pos + 1);
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day8 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let program = match parse_program(&input) {
        Ok(program) => program,
        Err(line_index) => {
            println!(
                "error: {}|{}| Failed to parse {}",
                filename,
                line_index + 1,
                input.lines().nth(line_index).unwrap()
            );
            return;
        }
    };
    let answer1 = solve_part1(&program);
    println!("Answer 1: {:?}", answer1);

    let answer2 = solve_part2(&program);
    println!("Answer 2: {:?}", answer2);
}

#[cfg(test)]
mod tests8 {
    use super::*;

    const EXAMPLE1: &str = "nop +0
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
        let program = parse_program(EXAMPLE1).expect("Failed to parse program");
        assert_eq!(solve_part1(&program), Some(5));
    }

    #[test]
    fn test2_1() {
        let program = parse_program(EXAMPLE1).expect("Failed to parse program");
        assert_eq!(solve_part2(&program), Some((8, 7)));
    }
}
