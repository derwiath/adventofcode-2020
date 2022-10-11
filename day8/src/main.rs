use gameboy;
use std::env;
use std::fs;

fn solve_part1(program: &Vec<gameboy::Instruction>) -> Option<i64> {
    match gameboy::execute_program(program) {
        Ok(accumulator) => Some(accumulator),
        Err(gameboy::ExitError::InvalidPC(_, _)) => None,
        Err(gameboy::ExitError::InfiniteLoop(accumulator, _)) => Some(accumulator),
    }
}

fn solve_part2(program: &Vec<gameboy::Instruction>) -> Option<(i64, usize)> {
    let mut next_pos: Option<usize> = program.iter().position(|i| match i {
        gameboy::Instruction::Nop(_) => true,
        _ => false,
    });
    while next_pos.is_some() {
        let mut copy = program.clone();
        let pos = next_pos.unwrap();
        let arg = match copy.get(pos) {
            Some(gameboy::Instruction::Nop(arg)) => arg,
            _ => {
                panic!("wtf");
            }
        };
        println!(
            "{}: {:?} -> {:?}",
            pos,
            copy[pos],
            gameboy::Instruction::Jmp(*arg as isize)
        );
        copy[pos] = gameboy::Instruction::Jmp(*arg as isize);

        match gameboy::execute_program(&copy) {
            Ok(accumulator) => return Some((accumulator, pos)),
            _ => (),
        }
        next_pos = program
            .iter()
            .skip(pos + 1)
            .position(|i| match i {
                gameboy::Instruction::Nop(_) => true,
                _ => false,
            })
            .map(|found_pos| found_pos + pos + 1);
    }

    let mut next_pos = program.iter().position(|instr| match instr {
        gameboy::Instruction::Jmp(_) => true,
        _ => false,
    });
    println!("{:?}", next_pos);
    while next_pos.is_some() {
        let mut copy = program.clone();
        let pos = next_pos.unwrap();
        let arg = match copy.get(pos) {
            Some(gameboy::Instruction::Jmp(arg)) => arg,
            Some(x) => {
                panic!("{}: wtf {:?}", pos, x);
            }
            None => {
                panic!("wtf none");
            }
        };
        println!(
            "{}: {:?} -> {:?}",
            pos + 1,
            copy[pos],
            gameboy::Instruction::Nop(*arg as i64)
        );
        copy[pos] = gameboy::Instruction::Nop(*arg as i64);

        match gameboy::execute_program(&copy) {
            Ok(accumulator) => return Some((accumulator, pos)),
            _ => (),
        }
        next_pos = program
            .iter()
            .skip(pos + 1)
            .position(|i| match i {
                gameboy::Instruction::Jmp(_) => true,
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

    let program = match gameboy::parse_program(&input) {
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
        let program = gameboy::parse_program(EXAMPLE1).expect("Failed to parse program");
        assert_eq!(solve_part1(&program), Some(5));
    }

    #[test]
    fn test2_1() {
        let program = gameboy::parse_program(EXAMPLE1).expect("Failed to parse program");
        assert_eq!(solve_part2(&program), Some((8, 7)));
    }
}
