//! Day 4: Giant Squid

use aoc2021::Input;
use std::io::{Error, ErrorKind, Result};

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

    // Parse the numbers that get called.
    let nums: Vec<usize> = input
        .lines()
        .unwrap()
        .iter()
        .nth(0)
        .map(|s| {
            s.split(",")
                .map(|v| usize::from_str_radix(v, 10).unwrap())
                .collect()
        })
        .unwrap();
    //dbg!(nums);

    // Parse the bingo cards.
    let mut boards: Vec<Board> = vec![];
    let lines: Vec<String> = input
        .lines()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.to_string())
        .collect();
    let mut skip: usize = 0;
    loop {
        let lns: Vec<Vec<isize>> = lines
            .iter()
            .skip(skip)
            .take(5)
            .map(|s| {
                s.split_whitespace()
                    .map(|v| isize::from_str_radix(v, 10).unwrap())
                    .collect()
            })
            .collect();

        if lns.len() < 5 {
            break;
        }

        let b = Board::from(lns);
        boards.push(b);
        skip += 5;
    }
    //dbg!(&cards);

    // Part 1: Find the board that will win first.
    println!("{}", winning_board_score(&nums, &boards));

    // Part 2: Find the board that will win last.
    println!("{}", last_winning_board_score(&nums, &boards));

    Ok(())
}

#[derive(Debug, Clone)]
struct Board {
    nums: Vec<Vec<isize>>,
}

impl From<Vec<Vec<isize>>> for Board {
    fn from(v: Vec<Vec<isize>>) -> Self {
        Self { nums: v }
    }
}

impl Board {
    fn mark(&mut self, n: isize) {
        let nums = self
            .nums
            .iter()
            .map(|row| row.iter().map(|&v| if v == n { -1 } else { v }).collect())
            .collect();
        self.nums = nums;
    }

    fn won(&self) -> bool {
        for i in 0..self.nums.len() {
            let mut rowp = true;
            let mut colp = true;
            for j in 0..self.nums[i].len() {
                if self.nums[i][j] != -1 {
                    rowp = false;
                }
                if self.nums[j][i] != -1 {
                    colp = false;
                }
            }
            if rowp || colp {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        let mut sum: usize = 0;
        for row in &self.nums {
            for &v in row {
                if v > -1 {
                    sum += v as usize;
                }
            }
        }
        sum
    }
}

fn winning_board_score(nums: &[usize], boards: &[Board]) -> usize {
    let mut boards = boards.to_vec();
    for n in nums {
        for b in boards.iter_mut() {
            b.mark(*n as isize);
            if b.won() {
                return b.score() * n;
            }
        }
    }
    0
}

fn last_winning_board_score(nums: &[usize], boards: &[Board]) -> usize {
    use std::collections::HashMap;
    let mut won: HashMap<usize, usize> = HashMap::new();
    let mut score: usize = 0;
    let mut boards = boards.to_vec();
    for n in nums {
        for (i, b) in boards.iter_mut().enumerate() {
            if let Some(_) = won.get(&i) {
                continue;
            }

            b.mark(*n as isize);
            if b.won() {
                score = b.score() * n;
                won.insert(i, score);
                eprintln!("board {} -> score {}", i, score);
            }
        }
    }

    score
}
