#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fmt;
use std::fs;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct Mask {
    set: u64,
    clear: u64,
}

#[allow(dead_code)]
impl Mask {
    pub fn new(set: u64, clear: u64) -> Self {
        Self { set, clear }
    }

    pub fn parse(s: &str) -> Option<Mask> {
        lazy_static! {
            static ref MASK_RE: regex::Regex = regex::Regex::new(r"mask = ([X01]*)").unwrap();
        }

        if let Some(captures) = MASK_RE.captures(s) {
            if captures.len() != 2 {
                return None;
            }

            let mask = captures.get(1).unwrap().as_str();
            let mut set = 0;
            let mut clear = 0;
            for c in mask.chars() {
                set <<= 1;
                clear <<= 1;
                match c {
                    '0' => {
                        clear |= 0x1;
                    }
                    '1' => {
                        set |= 0x1;
                    }
                    'X' => (),
                    _ => panic!("oh noh"),
                }
            }
            Some(Mask::new(set, !clear))
        } else {
            None
        }
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..36 {
            let c = if (self.set & 0x1 << i) != 0 {
                '1'
            } else if (self.clear & 0x1 << i) != 0 {
                '0'
            } else {
                'X'
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct Write {
    address: u64,
    value: u64,
}

#[allow(dead_code)]
impl Write {
    pub fn new(address: u64, value: u64) -> Self {
        Self { address, value }
    }

    pub fn parse(s: &str) -> Option<Write> {
        lazy_static! {
            static ref MEM_RE: regex::Regex = regex::Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
        }

        if let Some(captures) = MEM_RE.captures(s) {
            if captures.len() != 3 {
                return None;
            }
            let address = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            Some(Write::new(address, value))
        } else {
            None
        }
    }
}

impl fmt::Display for Write {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] = {}", self.address, self.value)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    SetMask(Mask),
    WriteValue(Write),
}

#[allow(dead_code)]
impl Instruction {
    pub fn parse(s: &str) -> Option<Instruction> {
        if let Some(mask) = Mask::parse(s) {
            Some(Self::SetMask(mask))
        } else if let Some(write) = Write::parse(s) {
            Some(Self::WriteValue(write))
        } else {
            None
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::SetMask(mask) => write!(f, "{}", mask),
            Instruction::WriteValue(value) => write!(f, "{}", value),
        }
    }
}

fn solve_part1(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d*) ([a-z]*)").unwrap();
    }
    let mut sum = 0;
    for line in input.lines() {
        if let Some(captures) = RE.captures(line) {
            if captures.len() == 3 {
                let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let _thing = captures.get(2).unwrap().as_str();
                sum += count;
            }
        }
    }
    sum
}

fn solve_part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const PACKAGE_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    let filename = args
        .get(1)
        .expect(format!("Usage: {} input-filename", PACKAGE_NAME.unwrap()).as_str());

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests14 {
    use super::*;

    const EXAMPLE1: &str = "
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
        mem[8] = 11\n\
        mem[7] = 101\n\
        mem[8] = 0";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 165);
    }

    #[test]
    fn test1_mask1() {
        let example = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        assert_eq!(Mask::parse(example), Some(Mask::new(0x40, 0x2)));
    }

    #[test]
    fn test1_write1() {
        let example = "mem[8] = 11";
        assert_eq!(Write::parse(example), Some(Write::new(8, 11)));
    }

    #[test]
    fn test1_write2() {
        let example = "mem[7] = 101";
        assert_eq!(Write::parse(example), Some(Write::new(7, 101)));
    }

    #[test]
    fn test1_write3() {
        let example = "mem[8] = 0";
        assert_eq!(Write::parse(example), Some(Write::new(8, 0)));
    }

    #[test]
    fn test1_instruction1() {
        let example = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        assert_eq!(
            Instruction::parse(example),
            Some(Instruction::SetMask(Mask::new(0x40, 0x2)))
        );
    }

    #[test]
    fn test1_instruction2() {
        let example = "mem[8] = 11";
        assert_eq!(
            Instruction::parse(example),
            Some(Instruction::WriteValue(Write::new(8, 11)))
        );
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
