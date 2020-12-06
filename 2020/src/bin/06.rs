//! Day 6: Custom Customs

use aoc2020::Input;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    let infile: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(infile)?;
    let answers = collect_group_answers(&input);

    // Part 1: Sum the number of answers from all groups.
    let n: usize = answers.iter().map(|a| a.anyones_answers()).sum();
    println!("Part 1: {}", n);

    let n: usize = answers.iter().map(|a| a.everyones_answers()).sum();
    println!("Part 2: {}", n);

    Ok(())
}

fn collect_group_answers(input: &Input) -> Vec<GroupAnswers> {
    let mut v = vec![];
    let mut buf = vec![];
    for line in input.raw_lines().unwrap() {
        if line.len() == 0 {
            let answers = GroupAnswers::from(buf.clone());
            v.push(answers);
            buf.clear();
        } else {
            buf.push(line);
        }
    }
    let answers = GroupAnswers::from(buf);
    v.push(answers);
    v
}

#[derive(Clone)]
struct GroupAnswers {
    group_size: usize,
    answers: HashMap<char, usize>,
}

impl From<Vec<String>> for GroupAnswers {
    fn from(v: Vec<String>) -> Self {
        let group_size = v.len();
        let mut answers = HashMap::new();
        for s in v {
            for c in s.chars() {
                if answers.contains_key(&c) {
                    if let Some(n) = answers.get_mut(&c) {
                        *n += 1
                    }
                } else {
                    answers.insert(c, 1);
                }
            }
        }
        Self {
            group_size,
            answers,
        }
    }
}

impl GroupAnswers {
    /// Returns the number of questions anyone answered "yes" to.
    fn anyones_answers(&self) -> usize {
        self.answers.values().count()
    }

    /// Returns the number of questions everyone answered "yes" to.
    fn everyones_answers(&self) -> usize {
        self.answers
            .values()
            .filter(|n| **n == self.group_size)
            .count()
    }
}

#[test]
fn test_anyones_answers() {
    let raw_input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let input = Input::from(raw_input);
    let groups = collect_group_answers(&input);

    assert_eq!(groups.len(), 5);

    assert_eq!(groups[0].anyones_answers(), 3);
    assert_eq!(groups[1].anyones_answers(), 3);
    assert_eq!(groups[2].anyones_answers(), 3);
    assert_eq!(groups[3].anyones_answers(), 1);
    assert_eq!(groups[4].anyones_answers(), 1);
}

#[test]
fn test_everyones_answers() {
    let raw_input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let input = Input::from(raw_input);
    let groups = collect_group_answers(&input);

    assert_eq!(groups.len(), 5);

    assert_eq!(groups[0].everyones_answers(), 3);
    assert_eq!(groups[1].everyones_answers(), 0);
    assert_eq!(groups[2].everyones_answers(), 1);
    assert_eq!(groups[3].everyones_answers(), 1);
    assert_eq!(groups[4].everyones_answers(), 1);
}
