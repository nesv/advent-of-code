use aoc::{
    intcode::{Program, Stop},
    Input,
};
use std::collections::HashMap;
use std::fmt;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let in_file: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(in_file)?;
    let code = input.to_string().unwrap();
    let mut program = Program::from(code.as_str());

    // Part 1
    let output = Output::from(program.execute()?);
    println!("{}", output);
    println!("Number of block tiles: {}", output.num_blocks());

    // Part 2
    program.reset(code.as_str()).set_mem(0, 2);
    program.input(0);
    loop {
        let output = Output::from(program.execute()?);
        match program.reason_for_stop().unwrap() {
            Stop::HCF => {
                println!("{}", output);
                break;
            }
            Stop::WaitingForInput => {
                if output.ball_x < output.paddle_x {
                    program.input(-1);
                } else if output.ball_x > output.paddle_x {
                    program.input(1);
                } else {
                    program.input(0);
                }
            }
        }
    }

    Ok(())
}

struct Output {
    map: HashMap<Object, Tile>,
    score: usize,
    ball_x: i64,
    paddle_x: i64,
}

impl From<Vec<i64>> for Output {
    fn from(v: Vec<i64>) -> Self {
        let mut score: usize = 0;
        let mut ball_x = 0;
        let mut paddle_x = 0;
        let mut map = HashMap::new();

        for (i, _) in v.iter().enumerate().step_by(3) {
            let x = v[i];
            let y = v[i + 1];

            if x == -1 && y == 0 {
                // Update the score.
                score = v[i + 2] as usize;
                continue;
            }

            let tile = Tile::from(v[i + 2]);
            match tile {
                Tile::Ball => {
                    ball_x = v[i];
                }
                Tile::HorizontalPaddle => {
                    paddle_x = v[i];
                }
                _ => {}
            }

            map.insert(Object { x, y }, tile);
        }

        Self {
            map,
            score,
            ball_x,
            paddle_x,
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.score)?;

        let mut max_y = 0;
        let mut max_x = 0;
        for (obj, _tile) in &self.map {
            if obj.y > max_y {
                max_y = obj.y;
            }
            if obj.x > max_x {
                max_x = obj.x;
            }
        }
        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                if let Some(tile) = self.map.get(&Object { x, y }) {
                    write!(f, "{}", tile)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Output {
    fn num_blocks(&self) -> i16 {
        self.map
            .iter()
            .map(|(_obj, tile)| match tile {
                Tile::Block => 1,
                _ => 0,
            })
            .sum()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Object {
    x: i64,
    y: i64,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,

    Unknown,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(f, "\u{2588}"),
            Tile::Block => write!(f, "\u{2591}"),
            Tile::HorizontalPaddle => write!(f, "\u{2581}"),
            Tile::Ball => write!(f, "\u{2299}"),
            Tile::Unknown => write!(f, "?"),
        }
    }
}

impl From<i64> for Tile {
    fn from(n: i64) -> Self {
        match n {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            _ => Self::Unknown,
        }
    }
}
