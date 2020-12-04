#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

#[derive(Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: usize,
    hcl: usize,
    ecl: usize,
    pid: usize,
    cid: usize,

    total: usize,
}

impl Passport {
    fn new() -> Self {
        Self {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: 0,
            hcl: 0,
            ecl: 0,
            pid: 0,
            cid: 0,
            total: 0,
        }
    }
    fn add(&mut self, field: &str, value: &str) {
        lazy_static! {
            static ref DIGIT_RE: regex::Regex = regex::Regex::new(r"(\d*)").unwrap();
            static ref HGT_RE: regex::Regex = regex::Regex::new(r"(\d*)([a-z]*)").unwrap();
            static ref HCL_RE: regex::Regex = regex::Regex::new(r"#([0-9a-f]*)").unwrap();
        }
        self.total += 1;
        match field {
            "byr" => {
                let captures = DIGIT_RE.captures(value).unwrap();
                if captures.len() == 2 {
                    let digit = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    if 1920 <= digit && digit <= 2002 {
                        self.byr += 1;
                    }
                }
            }
            "iyr" => {
                let captures = DIGIT_RE.captures(value).unwrap();
                if captures.len() == 2 {
                    let digit = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    if digit >= 2010 && digit <= 2020 {
                        self.iyr += 1
                    }
                }
            }
            "eyr" => {
                let captures = DIGIT_RE.captures(value).unwrap();
                if captures.len() == 2 {
                    let digit = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    if digit >= 2020 && digit <= 2030 {
                        self.eyr += 1
                    }
                }
            }
            "hgt" => {
                let captures = HGT_RE.captures(value).unwrap();
                if captures.len() == 3 {
                    let digit = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let unit = captures.get(2).unwrap().as_str();
                    if unit == "cm" && digit >= 150 && digit <= 193 {
                        self.hgt += 1
                    } else if unit == "in" && digit >= 59 && digit <= 76 {
                        self.hgt += 1
                    }
                }
            }
            "hcl" => match HCL_RE.captures(value) {
                Some(captures) => {
                    if captures.len() == 2 {
                        let digit = captures.get(1).unwrap().as_str();
                        if digit.len() == 6 {
                            self.hcl += 1;
                        }
                    }
                }
                None => (),
            },
            "ecl" => {
                for valid in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter() {
                    if &value == valid {
                        self.ecl += 1;
                        break;
                    }
                }
            }
            "pid" => {
                let captures = DIGIT_RE.captures(value).unwrap();
                if captures.len() == 2 {
                    let digit = captures.get(1).unwrap().as_str();
                    if digit.len() == 9 {
                        self.pid += 1;
                    }
                }
            }
            "cid" => self.cid += 1,
            _ => (),
        }
    }

    fn is_valid(&self) -> bool {
        self.byr > 0
            && self.iyr > 0
            && self.eyr > 0
            && self.hgt > 0
            && self.hcl > 0
            && self.ecl > 0
            && self.pid > 0
            && self.total > 0
    }
}

fn count_valid_passwords(s: &str) -> usize {
    let mut passport = Passport::new();
    let mut valid_count = 0;
    for line in s.lines() {
        if line.len() == 0 {
            if passport.is_valid() {
                valid_count += 1
            }
            passport = Passport::new()
        } else {
            for kv in line.split(' ') {
                let parts: Vec<&str> = kv.split(':').collect();
                let key = parts.get(0).unwrap();
                let value = parts.get(1).unwrap();
                passport.add(key, value);
            }
        }
    }
    if passport.is_valid() {
        valid_count += 1
    }
    valid_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day4 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");
    let valid_count = count_valid_passwords(&input);
    println!("Valid: {}", valid_count);
}

#[cfg(test)]
mod tests4 {
    use super::*;

    const EXAMPLE: &str = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";

    const ALL_INVALID: &str = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const ALL_VALID: &str = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
    const ONE_VALID: &str = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f
";

    #[test]
    fn test_1() {
        assert_eq!(count_valid_passwords(EXAMPLE), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(count_valid_passwords(ALL_INVALID), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(count_valid_passwords(ALL_VALID), 4);
    }

    #[test]
    fn test_4() {
        assert_eq!(count_valid_passwords(ONE_VALID), 1);
    }
}
