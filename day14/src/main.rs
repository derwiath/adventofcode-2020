#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections;
use std::env;
use std::fmt;
use std::fs;

struct FloatingAddress<'a> {
    address: u64,
    floating_indices: &'a [usize],
    floating_end_it: u64,
    floating_it: u64,
}

impl<'a> FloatingAddress<'a> {
    fn new(address: u64, floating_indices: &'a [usize]) -> FloatingAddress<'a> {
        let floating_end_it = 1 << floating_indices.len() as u64;
        FloatingAddress {
            address,
            floating_indices,
            floating_end_it,
            floating_it: 0,
        }
    }
}

impl<'a> Iterator for FloatingAddress<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.floating_it >= self.floating_end_it {
            return None;
        }
        let mut floating_set = 0;
        let mut floating_clear = 0;
        for (i, floating_index) in self.floating_indices.iter().enumerate() {
            floating_set |= ((self.floating_it >> i) & 0x1) << floating_index;
            floating_clear |= 0x1 << floating_index;
        }

        self.floating_it += 1;
        Some((self.address & !floating_clear) | floating_set)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct Mask {
    set: u64,
    clear: u64,
    floating_indices: Vec<usize>,
}

#[allow(dead_code)]
impl Mask {
    pub fn new(set: u64, clear: u64, floating_indices: Vec<usize>) -> Self {
        Self {
            set,
            clear,
            floating_indices,
        }
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
            let mut floating_indices = Vec::<usize>::new();
            let last_float_bit = mask.chars().count() - 1;
            for (i, c) in mask.chars().enumerate() {
                set <<= 1;
                clear <<= 1;
                match c {
                    '0' => {
                        clear |= 0x1;
                    }
                    '1' => {
                        set |= 0x1;
                    }
                    'X' => {
                        floating_indices.push(last_float_bit - i);
                    }
                    _ => panic!("oh noh"),
                }
            }
            floating_indices.reverse();
            Some(Mask::new(set, clear, floating_indices))
        } else {
            None
        }
    }

    pub fn apply_value(&self, value: u64) -> u64 {
        (value | self.set) & !self.clear
    }

    pub fn apply_address(&self, address: u64) -> FloatingAddress {
        FloatingAddress::new(address | self.set, &self.floating_indices[..])
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

fn solve_part1(input: &str) -> u64 {
    let mut mem = collections::HashMap::<u64, u64>::new();
    let mut mask = Mask::new(0, 0, Vec::<usize>::new());
    for line in input.lines().filter(|l| l.len() > 0) {
        match Instruction::parse(line) {
            Some(Instruction::SetMask(m)) => {
                mask = m;
            }
            Some(Instruction::WriteValue(write)) => {
                let value = mask.apply_value(write.value);
                mem.insert(write.address, value);
            }
            _ => panic!("Fail to parse: {}", line),
        }
    }
    mem.iter().map(|(_, value)| value).sum()
}

fn solve_part2(input: &str) -> u64 {
    let mut mem = collections::HashMap::<u64, u64>::new();
    let mut mask = Mask::new(0, 0, Vec::<usize>::new());
    for line in input.lines().filter(|l| l.len() > 0) {
        match Instruction::parse(line) {
            Some(Instruction::SetMask(m)) => {
                mask = m;
            }
            Some(Instruction::WriteValue(write)) => {
                for address in mask.apply_address(write.address) {
                    mem.insert(address, write.value);
                }
            }
            _ => panic!("Fail to parse: {}", line),
        }
    }
    mem.iter().map(|(_, value)| value).sum()
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

    const EXAMPLE1: &str = "\
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
        let floating_indices: Vec<usize> = (0..36).filter(|i| *i != 1 && *i != 6).collect();
        assert_eq!(
            Mask::parse(example),
            Some(Mask::new(0x40, 0x2, floating_indices))
        );
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
        let floating_indices: Vec<usize> = (0..36).filter(|i| *i != 1 && *i != 6).collect();
        assert_eq!(
            Instruction::parse(example),
            Some(Instruction::SetMask(Mask::new(0x40, 0x2, floating_indices)))
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

    const EXAMPLE2: &str = "\
        mask = 000000000000000000000000000000X1001X\n\
        mem[42] = 100\n\
        mask = 00000000000000000000000000000000X0XX\n\
        mem[26] = 1";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 208);
    }

    #[test]
    fn test2_2() {
        const EXAMPLE: &str = "mask = 000000000000000000000000000000X1001X";
        let mask = Mask::parse(EXAMPLE);
        let floating_indices = vec![0, 5];
        assert_eq!(mask, Some(Mask::new(0x12, 0xfffffffcc, floating_indices)));
        let mask = mask.unwrap();
        let mut addresses = mask.apply_address(42);
        assert_eq!(addresses.next(), Some(26));
        assert_eq!(addresses.next(), Some(27));
        assert_eq!(addresses.next(), Some(58));
        assert_eq!(addresses.next(), Some(59));
    }
}
