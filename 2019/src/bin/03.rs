use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

#[derive(Debug, PartialEq)]
enum Direction {
    Unknown(String),
    Right(i64),
    Left(i64),
    Up(i64),
    Down(i64),
}

impl Direction {
    fn new(s: &str) -> Self {
        let v: i64 = s[1..].parse().unwrap_or(0);
        match s.chars().nth(0).unwrap_or('x') {
            'R' => Self::Right(v),
            'L' => Self::Left(v),
            'U' => Self::Up(v),
            'D' => Self::Down(v),
            _ => Self::Unknown(String::from(s)),
        }
    }
}

#[derive(PartialEq)]
enum Orientation {
    Colinear,
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(i64, i64);

impl Point {
    /// Distance calculates the Manhattan distance to the other Point.
    fn distance_from(&self, p: &Point) -> i64 {
        let dx = self.0 - p.0;
        let dy = self.1 - p.1;
        dx.abs() + dy.abs()
    }

    fn on_segment(&self, s: &LineSegment) -> bool {
        (self.0 <= max(s.start.0, s.end.0) && self.0 >= min(s.start.0, s.end.0)) &&
            (self.1 <= max(s.start.1, s.end.1) && self.1 >= min(s.start.1, s.end.1))
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn intersection(&self, other: &LineSegment) -> Option<Point> {
        if !self.intersects(other) {
            return None;
        }

        // Figure out exactly which point intersects between the line segments.
        let a1 = self.end.1 - self.start.1;
        let b1 = self.start.0 - self.end.0;
        let c1 = (a1 * self.start.0) + (b1 * self.end.1);

        let a2 = other.end.1 - other.start.1;
        let b2 = other.start.0 - other.end.0;
        let c2 = (a2 * other.start.0) + (b2 * other.start.1);

        let determinant = (a1 * b2) - (a2 * b1);
        if determinant == 0 {
            // Lines are parallel.
            return None;
        }

        let x = ((b2 * c1) - (b1 * c2)) / determinant;
        let y = ((a1 * c2) - (a2 * c1)) / determinant;
        Some(Point(x, y))
    }

    fn intersects(&self, other: &LineSegment) -> bool {
        let o1 = self.orientation(&other.start);
        let o2 = self.orientation(&other.end);
        let o3 = other.orientation(&self.start);
        let o4 = other.orientation(&self.end);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        if o1 == Orientation::Colinear && other.start.on_segment(self) {
            return true;
        }
        if o2 == Orientation::Colinear && other.end.on_segment(self) {
            return true;
        }
        if o3 == Orientation::Colinear && self.start.on_segment(other) {
            return true;
        }
        if o4 == Orientation::Colinear && self.end.on_segment(other) {
            return true;
        }

        false
    }

    fn orientation(&self, q: &Point) -> Orientation {
        let p = self.start.clone();
        let r = self.end.clone();
        let v = (q.1 - p.1) * (r.0 - p.0) - (q.0 - p.0) * (r.1 - q.1);
        if v == 0 {
            return Orientation::Colinear;
        } else if v > 0 {
            return Orientation::Clockwise;
        }
        Orientation::CounterClockwise
    }
}


fn main() -> Result<()> {
    let in_file: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("Specify a path to the input file");
            std::process::exit(1);
        }
    };
    let file = File::open(in_file)?;

    // Collect the directions into line segments.
    let mut paths: Vec<Vec<LineSegment>> = vec![];
    let mut directions: Vec<Vec<Direction>> = vec![];
    for line in BufReader::new(file).lines() {
        let line = line.unwrap_or("".to_string());
        let dirs = parse_input_line(&line)?;
        let segs = convert_directions_to_segments(&dirs);
        paths.push(segs);
        directions.push(dirs);
    }

    let distance = part1(&paths)?;
    println!("{}", distance);

    let steps = part2(&directions)?;
    println!("{}", steps);

    Ok(())
}

#[test]
fn test_parse_input_line() {
    let line = String::from("R8,U5,L5,D3");
    let res = parse_input_line(&line).unwrap_or(vec![]);
    assert_eq!(res.len(), 4);
    assert_eq!(res[0], Direction::Right(8));
    assert_eq!(res[1], Direction::Up(5));
    assert_eq!(res[2], Direction::Left(5));
    assert_eq!(res[3], Direction::Down(3));
}

/// Returns a `Vec<Direction>` from a line of puzzle input.
/// Each line is expected to be a comma-separated list of movement instructions
/// like so:
///
/// ```
/// R123,U3,D74
/// ```
fn parse_input_line(line: &str) -> Result<Vec<Direction>> {
    if line.len() == 0 {
        return Err(Error::new(ErrorKind::Other, "zero-length input"));
    }

    let input: Vec<String> = line
        .trim_end()
        .split(",")
        .map(|s| String::from(s))
        .collect();

    let mut dirs: Vec<Direction> = vec![];
    for i in input {
        let d = Direction::new(&i);
        match d {
            Direction::Unknown(v) => {
                return Err(Error::new(ErrorKind::Other, v.as_str()));
            },
            _ => { dirs.push(d); },
        };
    }
    Ok(dirs)
}

fn convert_directions_to_segments(directions: &Vec<Direction>) -> Vec<LineSegment> {
    let mut segments: Vec<LineSegment> = vec![];
    let mut start = Point(0, 0);
    for d in directions {
        let end = go(&d, start);
        segments.push(LineSegment{
            start,
            end,
        });
        start = end;
    }
    segments
}

/// Returns the next Point for a wire's path, given a direction, and a starting
/// point.
fn go(direction: &Direction, from: Point) -> Point {
    match direction {
        Direction::Right(n) => Point(from.0 + n, from.1),
        Direction::Left(n) => Point(from.0 - n, from.1),
        Direction::Up(n) => Point(from.0, from.1 + n),
        Direction::Down(n) => Point(from.0, from.1 - n),
        Direction::Unknown(_) => Point(0, 0),
    }
}

#[test]
fn test_part1() {
    let raw_input: Vec<String> = vec![
        "R8,U5,L5,D3".to_string(),
        "U7,R6,D4,L4".to_string(),
    ];
    let wires: Vec<Vec<LineSegment>> = raw_input.iter().map(|v| {
        let dirs = parse_input_line(v).unwrap();
        let segs = convert_directions_to_segments(&dirs);
        segs
    }).collect();
    let distance = part1(&wires).unwrap();
    assert_eq!(distance, 6);

    let raw_input: Vec<String> = vec![
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
    ];
    let wires: Vec<Vec<LineSegment>> = raw_input.iter().map(|v| {
        let dirs = parse_input_line(v).unwrap();
        let segs = convert_directions_to_segments(&dirs);
        segs
    }).collect();
    let distance = part1(&wires).unwrap();
    assert_eq!(distance, 159);

    let raw_input: Vec<String> = vec![
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
    ];
    let wires: Vec<Vec<LineSegment>> = raw_input.iter().map(|v| {
        let dirs = parse_input_line(v).unwrap();
        convert_directions_to_segments(&dirs)
    }).collect();
    let distance = part1(&wires).unwrap();
    assert_eq!(distance, 135);
}

/// Returns the point, and the Manhattan distance for the intersection of
/// any number of wires, that is closest to the origin.
fn part1(paths: &Vec<Vec<LineSegment>>) -> Result<i64> {
    let intersections = match find_intersections(paths) {
        Some(v) => v,
        None => { return Err(Error::new(ErrorKind::Other, "No intersections found.")); },
    };

    // Find the intersection that is closest to the central point (0, 0).
    let origin = Point(0, 0);
    let mut distance: i64 = i64::max_value();
    for i in intersections {
        let d = i.distance_from(&origin);
        if d < distance {
            distance = d;
        }
    }

    Ok(distance)
}

// Finds all intersections between the wires.
// Note that any intersections at `Point(0, 0)` are ignored.
fn find_intersections(wires: &Vec<Vec<LineSegment>>) -> Option<Vec<Point>> {
    let mut inters: Vec<Point> = vec![];
    for (i, wire) in wires.iter().enumerate() {
        if i + 1 == wires.len() {
            break;
        }

        let next_wire = &wires[i+1];
        for a in wire {
            for b in next_wire {
                if let Some(i) = a.intersection(&b) {
                    // Ignore the point if it's at (0, 0).
                    if i != Point(0, 0) {
                        inters.push(i);
                    }
                }
            }
        }
    }
    if inters.len() > 0 {
        return Some(inters);
    }
    None
}

#[test]
fn test_part2() {
    let raw_input: Vec<String> = vec![
        "R8,U5,L5,D3".to_string(),
        "U7,R6,D4,L4".to_string(),
    ];
    let directions: Vec<Vec<Direction>> = raw_input
        .iter()
        .map(|v| parse_input_line(v).unwrap())
        .collect();
    let steps = part2(&directions).unwrap();
    assert_eq!(steps, 30);

    let raw_input: Vec<String> = vec![
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
    ];
    let directions: Vec<Vec<Direction>> = raw_input
        .iter()
        .map(|v| parse_input_line(v).unwrap())
        .collect();
    let steps = part2(&directions).unwrap();
    assert_eq!(steps, 610);

    let raw_input: Vec<String> = vec![
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
    ];
    let directions: Vec<Vec<Direction>> = raw_input
        .iter()
        .map(|v| parse_input_line(v).unwrap())
        .collect();
    let steps = part2(&directions).unwrap();
    assert_eq!(steps, 410);
}

/// Find the intersections, but count the number of steps it took
/// for each wire to reach that intersection.
/// Sum the number of steps from each wire, and return the lowest sum.
fn part2(directions: &Vec<Vec<Direction>>) -> Result<i64> {
    let paths: Vec<Vec<LineSegment>> = directions
        .iter()
        .map(|dv| convert_directions_to_segments(dv))
        .collect();
    let intersections = match find_intersections(&paths) {
        Some(v) => v,
        None => { return Err(Error::new(ErrorKind::Other, "No intersections found.")); },
    };

    let mut steps_taken: HashMap<Point, Vec<i64>> = HashMap::new();
    for i in intersections {
        // The number of steps taken for each wire, to get to the
        // intersection `i`.
        let steps: Vec<i64> = directions
            .iter()
            .map(|dir| walk_path(dir, &i))
            .collect();

        // Associates the given intersection, with the number of steps taken
        // by each wire.
        steps_taken.insert(i.clone(), steps);
    }

    let mut lowest = i64::max_value();
    for (_p, steps) in steps_taken {
        let mut sum = 0;
        for s in &steps {
            sum += s;
        }
        if sum < lowest {
            lowest = sum;
        }
    }

    Ok(lowest)
}

/// Travels the distance of the given path, and counts the number of steps
/// taken until it reaches the given intersection.
fn walk_path(directions: &Vec<Direction>, intersection: &Point) -> i64 {
    let mut steps: i64 = 0;
    let mut start = Point(0, 0);
    for dir in directions {
        // Create a line segment for the current path we just took,
        // and check to see if the intersection is somewhere on that segment.
        let end = go(dir, start);
        let ls = LineSegment{start, end};
        if intersection.on_segment(&ls) {
            // Our destination is somewhere on this line segment.
            // Figure out how many steps were taken on this segment to get
            // to the destination.
            let n = intersection.distance_from(&start);
            return steps + n;
        }

        // Simply increment the number of steps we have taken.
        let n = end.distance_from(&start);
        steps += n;
        start = end;
    }
    steps
}

fn max(a: i64, b: i64) -> i64 {
    if a > b {
        return a;
    }
    b
}

fn min(a: i64, b: i64) -> i64 {
    if a < b {
        return a;
    }
    b
}
