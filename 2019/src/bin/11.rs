use aoc::{
    Input,
    intcode::{
        Program,
        Stop,
    },
};
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

    let panels_painted = part1(&code)?;
    println!("{}", panels_painted);

    part2(&code)?;

    Ok(())
}

/// Return the number of panels the robot painted at least once.
///
/// Provide input to the program to indicate what colour panel the robot
/// is currently over:
///
/// 0 = black
/// 1 = white
///
/// The program will output two values:
///
/// The first indicates the colour the panel should be painted (0=black, 1=white);
/// The second indicates the direction the robot should turn (0=left 90deg, 1=right 90deg).
///
/// After the robot turns, it should move forward one panel.
///
/// Do not restart the program between runs.
fn part1(code: &str) -> Result<usize> {
    use aoc::day10::Position;
    use std::collections::HashMap;

    let mut program = Program::from(code);
    let mut panels: HashMap<Position, Color> = HashMap::new();
    let mut position = Position::new(0, 0);
    let mut direction = Direction::Up;
    loop {
        // Get the current panel's colour.
        let current_color: i64 = match panels.get(&position).unwrap_or(&Color::Black) {
            Color::Black => 0,
            Color::White => 1,
        };

        // Run the program, providing the current panel's colour as input.
        program.input(current_color);
        let output = program.execute()?;
        if let Some(reason) = program.reason_for_stop() {
            match reason {
                Stop::HCF => { break; }
                _ => {}
            }
        }
        if output.len() == 0 {
            break;
        }
        
        assert_eq!(output.len(), 2);

        // Figure out which colour to paint the current panel.
        let color = match output[0] {
            0 => Color::Black,
            1 => Color::White,
            _ => { panic!("invalid color code: {}", output[0]); }
        };

        // "Paint" the current panel.
        panels.insert(position.clone(), color); 
        //eprintln!("Painted panel @ {:?} => {:?}", &position, &color);

        // Figure out which direction to turn, and move one step in that
        // direction.
        let dir = direction.turn(Turn::new(output[1]));
        position = match dir {
            Direction::Right => Position::new(position.x + 1, position.y),
            Direction::Left => Position::new(position.x - 1, position.y),
            Direction::Up => Position::new(position.x, position.y + 1),
            Direction::Down => Position::new(position.x, position.y - 1),
        };
        //eprintln!("{:?} => {:?}", &dir, &position);
        direction = dir;
    }

    Ok(panels.len())
}

/// Same as part 1, but the default panel colour is white.
/// Draw out the result of the what the robot painted.
fn part2(code: &str) -> Result<()> {
    use aoc::day10::Position;
    use std::collections::HashMap;

    let mut program = Program::from(code);
    let mut panels: HashMap<Position, Color> = HashMap::new();
    let mut position = Position::new(0, 0);
    let mut direction = Direction::Up;
    loop {
        // Get the current panel's colour.
        let current_color: i64 = match panels.get(&position).unwrap_or(&Color::White) {
            Color::Black => 0,
            Color::White => 1,
        };

        // Run the program, providing the current panel's colour as input.
        program.input(current_color);
        let output = program.execute()?;
        if let Some(reason) = program.reason_for_stop() {
            match reason {
                Stop::HCF => { break; }
                _ => {}
            }
        }
        if output.len() == 0 {
            break;
        }
        
        assert_eq!(output.len(), 2);

        // Figure out which colour to paint the current panel.
        let color = match output[0] {
            0 => Color::Black,
            1 => Color::White,
            _ => { panic!("invalid color code: {}", output[0]); }
        };

        // "Paint" the current panel.
        panels.insert(position.clone(), color); 
        //eprintln!("Painted panel @ {:?} => {:?}", &position, &color);

        // Figure out which direction to turn, and move one step in that
        // direction.
        let dir = direction.turn(Turn::new(output[1]));
        position = match dir {
            Direction::Right => Position::new(position.x + 1, position.y),
            Direction::Left => Position::new(position.x - 1, position.y),
            Direction::Up => Position::new(position.x, position.y + 1),
            Direction::Down => Position::new(position.x, position.y - 1),
        };
        //eprintln!("{:?} => {:?}", &dir, &position);
        direction = dir;
    }

    // Figure out the minimum X and Y coordinates (offsets), so we can draw
    // out what the robot painted assuming everything starts at (0, 0).
    let mut min_x: i32 = i32::max_value();
    let mut min_y: i32 = i32::max_value();
    let mut max_x: i32 = i32::min_value();
    let mut max_y: i32 = i32::min_value();
    for k in panels.keys() {
        if k.x < min_x {
            min_x = k.x;
        }
        if k.x > max_x {
            max_x = k.x;
        }
        if k.y < min_y {
            min_y = k.y;
        }
        if k.y > max_y {
            max_y = k.y;
        }
    }
    
    let mut canvas: Vec<Vec<Color>> = vec![];
    for y in (min_y..max_y + 1).rev() {
        for x in min_x..max_x + 1 {
            let pos = Position::new(x, y);
            match panels.get(&pos).unwrap_or(&Color::Black) {
                Color::Black => { print!(" "); }
                Color::White => { print!("\u{2588}"); }
            }
        }
        println!("");
    }

    Ok(())
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

#[derive(Eq, PartialEq, Debug)]
enum Turn {
    Right,
    Left,
}

impl Turn {
    fn new(n: i64) -> Self {
        match n {
            0 => Self::Left,
            1 => Self::Right,
            _ => { panic!("unacceptable value: {}", n); },
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    fn turn(&self, t: Turn) -> Self {
        match self {
            Self::Right => match t {
                Turn::Left => Self::Up,
                Turn::Right => Self::Down,
            }
            Self::Left => match t {
                Turn::Left => Self::Down,
                Turn::Right => Self::Up,
            }
            Self::Up => match t {
                Turn::Left => Self::Left,
                Turn::Right => Self::Right,
            }
            Self::Down => match t {
                Turn::Left => Self::Right,
                Turn::Right => Self::Left,
            }
        }
    }
}
