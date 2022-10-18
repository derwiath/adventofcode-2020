#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

// Inclusive range
#[derive(Debug, PartialEq)]
struct Range {
    min: usize,
    max: usize,
}

#[derive(Debug, PartialEq)]
struct Rule {
    label: String,
    range1: Range,
    range2: Range,
}

impl Range {
    fn new(min: usize, max: usize) -> Range {
        Range { min, max }
    }

    fn includes(&self, number: &usize) -> bool {
        self.min <= *number && *number <= self.max
    }
}

impl Rule {
    fn new(label: &str, range1: Range, range2: Range) -> Rule {
        Rule {
            label: label.to_owned(),
            range1,
            range2,
        }
    }
}

#[derive(Debug)]
enum ParseState {
    Rules,
    YourTicket,
    NearbyTickets,
    Eof,
}

#[allow(dead_code)]
struct TicketInfo {
    ranges: Vec<Range>,
    your_numbers: Vec<usize>,
    nearby_numbers: Vec<usize>,
}

impl TicketInfo {
    fn new(ranges: Vec<Range>, your_numbers: Vec<usize>, nearby_numbers: Vec<usize>) -> TicketInfo {
        TicketInfo {
            ranges,
            your_numbers,
            nearby_numbers,
        }
    }
}

fn parse_rule(line: &str) -> Result<Rule, ()> {
    lazy_static! {
        static ref RULES_RE: regex::Regex =
            regex::Regex::new(r"(.*): ([\d]*)-([\d]*) or ([\d]*)-([\d]*)").unwrap();
    }
    if let Some(captures) = RULES_RE.captures(line) {
        assert_eq!(captures.len(), 6);
        let label: &str = &captures[1];
        let numbers: Vec<usize> = captures
            .iter()
            .skip(2)
            .map(|s| s.unwrap().as_str().parse::<usize>().unwrap())
            .collect();
        Ok(Rule::new(
            label,
            Range::new(numbers[0], numbers[1]),
            Range::new(numbers[2], numbers[3]),
        ))
    } else {
        Err(())
    }
}

fn parse_ticket_info(input: &str) -> Result<TicketInfo, &'static str> {
    let mut parse_state = ParseState::Rules;
    let mut ranges = Vec::<Range>::new();
    let mut your_numbers = Vec::<usize>::new();
    let mut nearby_numbers = Vec::<usize>::new();
    for line in input.lines().skip_while(|l| l.len() == 0) {
        let next_parse_state: ParseState = match parse_state {
            ParseState::Rules => {
                if line.len() == 0 {
                    ParseState::YourTicket
                } else if let Ok(rule) = parse_rule(line) {
                    ranges.push(rule.range1);
                    ranges.push(rule.range2);
                    ParseState::Rules
                } else {
                    return Err("Failed to parse rule");
                }
            }
            ParseState::YourTicket => {
                if line == "your ticket:" {
                    ParseState::YourTicket
                } else if line.len() == 0 {
                    ParseState::NearbyTickets
                } else {
                    line.split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .for_each(|n| your_numbers.push(n));
                    ParseState::NearbyTickets
                }
            }
            ParseState::NearbyTickets => {
                if line == "nearby tickets:" {
                    ParseState::NearbyTickets
                } else if line.len() == 0 {
                    if nearby_numbers.len() > 0 {
                        ParseState::Eof
                    } else {
                        ParseState::NearbyTickets
                    }
                } else {
                    line.split(',')
                        .map(|s| (s.parse::<usize>().unwrap()))
                        .for_each(|n| nearby_numbers.push(n));
                    ParseState::NearbyTickets
                }
            }
            ParseState::Eof => {
                break;
            }
        };
        parse_state = next_parse_state
    }
    Ok(TicketInfo::new(ranges, your_numbers, nearby_numbers))
}

fn solve_part1(input: &str) -> usize {
    let ticket_info = match parse_ticket_info(input) {
        Ok(ticket_info) => ticket_info,
        Err(msg) => panic!("Error: {}", msg),
    };

    ticket_info
        .nearby_numbers
        .iter()
        .filter(|number| {
            ticket_info
                .ranges
                .iter()
                .find(|r| r.includes(number))
                .is_none()
        })
        .fold(0, |acc, invalid_number| acc + invalid_number)
}

fn solve_part2(input: &str) -> usize {
    let ticket_info = match parse_ticket_info(input) {
        Ok(ticket_info) => ticket_info,
        Err(msg) => panic!("Error: {}", msg),
    };

    ticket_info
        .your_numbers
        .iter()
        .filter(|number| {
            ticket_info
                .ranges
                .iter()
                .find(|r| r.includes(number))
                .is_some()
        })
        .fold(1, |acc, number| acc * number)
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
mod tests16 {
    use super::*;

    #[test]
    fn test1_parserule() {
        assert_eq!(
            parse_rule("class: 1-3 or 5-7"),
            Ok(Rule::new("class", Range::new(1, 3), Range::new(5, 7)))
        );
    }

    const EXAMPLE1: &str = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 71);
    }

    const EXAMPLE2: &str = "
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 12 * 11 * 13);
    }
}
