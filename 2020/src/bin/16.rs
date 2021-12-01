//! Day 16: Ticket Translation

use aoc2020::Input;
use std::{
    io::{Error, ErrorKind, Result},
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

fn main() -> Result<()> {
    let infile = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(infile)?;
    let input = parse_input(&input)?;

    // Part 1:
    // Collect the invalid ticket values, and add them together.
    // This will be your "ticket scanning error rate".
    println!("Part 1: {}", input.ticket_scanning_error_rate());

    // Part 2:
    // Figure out which rule applies to each field in the valid tickets.
    // The ticket field orders are consistent, so if the third field on a
    // ticket is the "class", the third field of every ticket is the "class".
    let valid = input.valid_tickets();
    dbg!(valid.len());

    Ok(())
}

fn parse_input(input: &Input) -> Result<PuzzleInput> {
    let mut rules = vec![];
    let mut skip: usize = 0;
    let mut your_ticket_idx: usize = 0;
    let lines = input.lines().unwrap();
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("your ticket:") {
            skip += 1;
            your_ticket_idx = i + 1;
            continue;
        } else if line.starts_with("nearby tickets:") {
            skip += 1;
            break;
        } else if line.chars().nth(0).unwrap().is_ascii_digit() {
            skip += 1;
            continue;
        } else {
            skip += 1;
            let rule = Rule::from_str(&line).unwrap();
            rules.push(rule);
        }
    }

    // Parse your ticket.
    let your_ticket: Vec<i32> = lines[your_ticket_idx]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // The rest of the file should be the other peoples' tickets.
    let mut nearby_tickets: Vec<Vec<i32>> = vec![];
    for line in lines.iter().skip(skip) {
        let ticket: Vec<i32> = line
            .split(",")
            .map(|s| match s.parse::<i32>() {
                Ok(n) => n,
                Err(e) => {
                    panic!("parse {}: {:?}", s, e);
                }
            })
            .collect();
        nearby_tickets.push(ticket);
    }

    Ok(PuzzleInput {
        rules,
        your_ticket,
        nearby_tickets,
    })
}

#[derive(Debug)]
struct PuzzleInput {
    rules: Vec<Rule>,
    your_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

impl PuzzleInput {
    /// Goes through all of the nearby tickets, to figure out how many tickets
    /// contain invalid field values.
    fn ticket_scanning_error_rate(&self) -> i32 {
        self.nearby_tickets
            .iter()
            .map(|t| self.is_ticket_valid(t))
            .filter(|v| match v {
                Valid::Yes => false,
                Valid::No(_) => true,
            })
            .fold(0, |acc, v| match v {
                Valid::No(n) => acc + n,
                _ => acc,
            })
    }

    fn valid_tickets(&self) -> Vec<Vec<i32>> {
        self.nearby_tickets
	    .clone()
            .into_iter()
            .filter(|t| match self.is_ticket_valid(t) {
                Valid::Yes => true,
                Valid::No(_) => false,
            })
            .collect()
    }

    fn is_ticket_valid(&self, ticket: &[i32]) -> Valid {
        for n in ticket {
            let invalid: Vec<Valid> = self
                .rules
                .iter()
                .map(|r| {
                    if r.contains(*n) {
                        Valid::Yes
                    } else {
                        Valid::No(*n)
                    }
                })
                .filter(|v| match v {
                    Valid::Yes => false,
                    Valid::No(_) => true,
                })
                .collect();
            if invalid.len() == ticket.len() {
                return Valid::No(*n);
            }
        }
        Valid::Yes
    }
}

#[derive(Debug, PartialEq)]
enum Valid {
    Yes,
    No(i32),
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<i32>>,
}

impl Rule {
    fn contains(&self, n: i32) -> bool {
        for r in &self.ranges {
            if r.contains(&n) {
                return true;
            }
        }
        false
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ":").map(|s| s.trim()).collect();
        let name = parts[0].to_string();
        let ranges: Vec<RangeInclusive<i32>> = parts[1]
            .splitn(2, "or")
            .map(|s| {
                let t: Vec<&str> = s.trim().splitn(2, "-").collect();
                let start: i32 = t[0].parse().unwrap();
                let end: i32 = t[1].parse().unwrap();
                RangeInclusive::new(start, end)
            })
            .collect();

        Ok(Self { name, ranges })
    }
}

#[test]
fn test_parse_input() {
    let raw = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let input = Input::from(raw);
    let input = parse_input(&input).unwrap();
    eprintln!("{:?}", input);

    assert_eq!(input.rules.len(), 3);

    assert_eq!(input.your_ticket, [7, 1, 14]);

    assert_eq!(input.nearby_tickets.len(), 4);
    let mut iter = input.nearby_tickets.iter();
    assert_eq!(iter.next(), Some(&[7, 3, 47].to_vec()));
    assert_eq!(iter.next(), Some(&[40, 4, 50].to_vec()));
    assert_eq!(iter.next(), Some(&[55, 2, 20].to_vec()));
    assert_eq!(iter.next(), Some(&[38, 6, 12].to_vec()));
}

#[test]
fn test_scanning_error_rate() {
    let raw = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let input = Input::from(raw);
    let input = parse_input(&input).unwrap();
    assert_eq!(input.ticket_scanning_error_rate(), 71);
}
