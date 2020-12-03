//! Day 3: Toboggan Trajectory

use aoc2020::Input;
use std::fmt;
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

    let map = Map::from(&input);

    // Part 1: How many trees are there on the given slope/path?
    let n = map
        .walk(3, 1)?
        .into_iter()
        .filter(|i| match i {
            Item::Tree => true,
            _ => false,
        })
        .collect::<Vec<Item>>()
        .len();
    println!("Number of trees: {}", n);

    // Part 2: For each of the listed slopes, figure out how many trees
    // would be encountered, and multiply the results together.
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees_encountered = vec![];
    for slope in slopes {
        let n = map
            .walk(slope.0, slope.1)?
            .into_iter()
            .filter(|i| match i {
                Item::Tree => true,
                _ => false,
            })
            .collect::<Vec<Item>>()
            .len();
        trees_encountered.push(n);
    }

    let mut result = 1;
    for n in trees_encountered {
        result *= n;
    }
    println!("Number of trees: {}", result);

    Ok(())
}

#[derive(Copy, Clone)]
enum Item {
    Nothing,
    Tree,
    Unknown(char),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Item::Nothing => '.',
                Item::Tree => '#',
                Item::Unknown(c) => *c,
            }
        )
    }
}

struct Map {
    coords: Vec<Vec<Item>>,
}

impl Map {
    // Traverse the map on the given slope, starting at (0, 0),
    // returning a vector of items that are encountered on that path.
    fn walk(&self, x: usize, y: usize) -> Result<Vec<Item>> {
        let height = self.coords.len();
        if height == 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "no rows in map"));
        }

        let width = self.coords[0].len();
        if width == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "zero length initial row",
            ));
        }

        let mut pos: (usize, usize) = (0, 0);
        let mut v = vec![];
        loop {
            pos = (pos.0 + x, pos.1 + y);
            if let Some(item) = self.at(pos.0, pos.1) {
                v.push(item);
            } else {
                break;
            }
        }

        Ok(v)
    }

    // Return the `Item` at the given position.
    // Note that items requested from a column wider than the map are
    // wrapped around.
    // This method will return `None` if `y` is greater-than the height
    // of the map.
    fn at(&self, x: usize, y: usize) -> Option<Item> {
        if y >= self.coords.len() {
            return None;
        }

        let x = x % self.coords[y].len();
        Some(self.coords[y][x].clone())
    }
}

impl From<&Input> for Map {
    fn from(input: &Input) -> Self {
        let mut y = vec![];
        for line in input.lines().unwrap() {
            let x = line
                .chars()
                .map(|c| match c {
                    '.' => Item::Nothing,
                    '#' => Item::Tree,
                    _ => Item::Unknown(c),
                })
                .collect();
            y.push(x);
        }
        Self { coords: y }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in &self.coords {
            for x in y {
                write!(f, "{}", x)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
