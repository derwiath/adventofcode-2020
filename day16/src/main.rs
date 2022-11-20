#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fmt;
use std::fs;

// Inclusive range
#[derive(Debug, PartialEq)]
struct Range {
    min: usize,
    max: usize,
}

#[derive(PartialEq)]
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

    fn includes(&self, number: &usize) -> bool {
        self.range1.includes(number) || self.range2.includes(number)
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rule ").field("label", &self.label).finish()
    }
}

#[derive(Debug)]
enum ParseState {
    Rules,
    YourTicket,
    NearbyTickets,
    Eof,
}

struct Ticket {
    numbers: Vec<usize>,
}

impl Ticket {
    fn new(numbers: Vec<usize>) -> Ticket {
        Ticket { numbers }
    }
}

#[allow(dead_code)]
struct TicketInfos {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl TicketInfos {
    fn new(rules: Vec<Rule>, your_ticket: Ticket, nearby_tickets: Vec<Ticket>) -> TicketInfos {
        TicketInfos {
            rules,
            your_ticket,
            nearby_tickets,
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

fn parse_ticket_infos(input: &str) -> Result<TicketInfos, &'static str> {
    let mut parse_state = ParseState::Rules;
    let mut rules = Vec::<Rule>::new();
    let mut your_ticket: Option<Ticket> = None;
    let mut nearby_tickets = Vec::<Ticket>::new();
    for line in input.lines().skip_while(|l| l.len() == 0) {
        let next_parse_state: ParseState = match parse_state {
            ParseState::Rules => {
                if line.len() == 0 {
                    ParseState::YourTicket
                } else if let Ok(rule) = parse_rule(line) {
                    rules.push(rule);
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
                    let numbers: Vec<usize> = line
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();
                    assert_eq!(your_ticket.is_none(), true);
                    your_ticket = Some(Ticket::new(numbers));
                    ParseState::NearbyTickets
                }
            }
            ParseState::NearbyTickets => {
                if line == "nearby tickets:" {
                    ParseState::NearbyTickets
                } else if line.len() == 0 {
                    if nearby_tickets.len() > 0 {
                        ParseState::Eof
                    } else {
                        ParseState::NearbyTickets
                    }
                } else {
                    let numbers: Vec<usize> = line
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();
                    nearby_tickets.push(Ticket::new(numbers));
                    ParseState::NearbyTickets
                }
            }
            ParseState::Eof => {
                break;
            }
        };
        parse_state = next_parse_state
    }
    if your_ticket.is_some() {
        Ok(TicketInfos::new(
            rules,
            your_ticket.unwrap(),
            nearby_tickets,
        ))
    } else {
        Err("your ticket not found")
    }
}

fn solve_part1(input: &str) -> usize {
    let ticket_infos = match parse_ticket_infos(input) {
        Ok(ticket_infos) => ticket_infos,
        Err(msg) => panic!("Error: {}", msg),
    };

    ticket_infos
        .nearby_tickets
        .iter()
        .flat_map(|nearby_ticket| &nearby_ticket.numbers)
        .filter(|number| {
            ticket_infos
                .rules
                .iter()
                .find(|r| r.includes(number))
                .is_none()
        })
        .fold(0, |acc, invalid_number| acc + invalid_number)
}

fn get_valid_tickets<'a>(ticket_infos: &'a TicketInfos) -> Vec<&'a Ticket> {
    ticket_infos
        .nearby_tickets
        .iter()
        .filter(|ticket| ticket.numbers.len() == ticket_infos.rules.len())
        .filter(|ticket| {
            ticket.numbers.iter().all(|number| {
                ticket_infos
                    .rules
                    .iter()
                    .find(|r| r.includes(number))
                    .is_some()
            })
        })
        .collect()
}

fn get_column_labels<'a>(ticket_infos: &'a TicketInfos) -> Vec<&'a str> {
    let valid_tickets = get_valid_tickets(&ticket_infos);

    let mut rules_map: HashMap<usize, Vec<&Rule>> = HashMap::new();
    for column in 0..ticket_infos.rules.len() {
        let rules: Vec<&Rule> = ticket_infos
            .rules
            .iter()
            .filter(|r| valid_tickets.iter().all(|t| r.includes(&t.numbers[column])))
            .collect();
        rules_map.insert(column, rules);
    }

    rules_map
        .iter()
        .for_each(|(column, rules)| println!("{} {:?}", column, rules));

    let mut single_rules_map: BTreeMap<usize, &Rule> = BTreeMap::new();
    loop {
        let single_rule_tickets: Vec<(usize, &Rule)> = rules_map
            .iter()
            .filter(|(_, rules)| rules.len() == 1)
            .map(|(column, rules)| (column.clone(), rules[0]))
            .collect();
        if single_rule_tickets.len() == 0 {
            break;
        }
        single_rule_tickets.iter().for_each(|(column, _)| {
            rules_map.remove(column);
        });
        single_rule_tickets.iter().for_each(|(column, taken_rule)| {
            let mut new_rules_map: HashMap<usize, Vec<&Rule>> = HashMap::new();
            for (column, rules) in &rules_map {
                new_rules_map.insert(
                    *column,
                    rules
                        .iter()
                        .filter(|r| r != &taken_rule)
                        .map(|r| *r)
                        .collect(),
                );
            }
            rules_map = new_rules_map;

            single_rules_map.insert(*column, *taken_rule);
        });
    }

    single_rules_map
        .iter()
        .map(|(_, rule)| rule.label.as_str())
        .collect()
}

fn solve_part2(input: &str) -> usize {
    let ticket_infos = match parse_ticket_infos(input) {
        Ok(ticket_infos) => ticket_infos,
        Err(msg) => panic!("Error: {}", msg),
    };

    let column_labels = get_column_labels(&ticket_infos);

    let departure_columns: Vec<usize> = column_labels
        .iter()
        .enumerate()
        .filter_map(|(column, label)| {
            if label.starts_with("departure") {
                Some(column)
            } else {
                None
            }
        })
        .collect();

    ticket_infos
        .your_ticket
        .numbers
        .iter()
        .enumerate()
        .filter(|(column, _)| departure_columns.iter().find(|x| *x == column).is_some())
        .fold(1, |acc, (_, value)| acc * value)
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
        let ticket_infos = parse_ticket_infos(EXAMPLE2).expect("Failed to parse ticket infos");
        assert_eq!(
            get_column_labels(&ticket_infos),
            vec!["row", "class", "seat"]
        );
    }
}
