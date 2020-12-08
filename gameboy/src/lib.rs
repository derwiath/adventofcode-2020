#[macro_use]
extern crate lazy_static;
extern crate regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(isize),
}

#[allow(dead_code)]
impl Instruction {
    pub fn parse(s: &str) -> Option<Instruction> {
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
pub enum ExitError {
    InfiniteLoop(i64, usize),
    InvalidPC(isize, usize),
}

pub fn parse_program(input: &str) -> Result<Vec<Instruction>, usize> {
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

pub fn execute_program(program: &Vec<Instruction>) -> Result<i64, ExitError> {
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

#[cfg(test)]
mod gameboy_tests {
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
}
