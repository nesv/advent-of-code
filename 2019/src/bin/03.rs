use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

use aoc::{Direction, Directions, Input, LineSegment, Point};

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

    // Collect the directions into line segments.
    let mut paths: Vec<Vec<LineSegment>> = vec![];
    let mut directions: Vec<Vec<Direction>> = vec![];
    match input.split_lines(",") {
        Some(lines) => {
            for line in lines {
                let mut dirs: Vec<Direction> = vec![];
                for l in line.into_iter() {
                    dirs.push(Direction::new(&l));
                }

                let mut path: Vec<LineSegment> = vec![];
                let mut start = Point::new(0, 0);
                for d in dirs.iter() {
                    let end = start.travel(&d);
                    path.push(LineSegment::new(start, end));
                    start = end;
                }

                directions.push(dirs);
                paths.push(path);
            }
        }
        None => {
            return Err(Error::new(ErrorKind::Other, "No data returned from input"));
        }
    }

    let distance = part1(&paths)?;
    println!("{}", distance);

    let steps = part2(&directions)?;
    println!("{}", steps);

    Ok(())
}

#[test]
fn test_part1() {
    let raw_input = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let input = Input::from(raw_input);
    let wires: Vec<Vec<LineSegment>> = input
        .split_lines(",")
        .unwrap()
        .iter()
        .map(|v| {
            let dirs = Directions::new(v.iter().map(|d| Direction::new(d)).collect());
            dirs.into()
        })
        .collect();
    let distance = part1(&wires).unwrap();
    assert_eq!(distance, 6);

    let raw_input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let input = Input::from(raw_input);
    let wires: Vec<Vec<LineSegment>> = input
        .split_lines(",")
        .unwrap()
        .iter()
        .map(|v| {
            let dirs = Directions::new(v.iter().map(|d| Direction::new(d)).collect());
            dirs.into()
        })
        .collect();
    let distance = part1(&wires).unwrap();
    assert_eq!(distance, 159);

    let raw_input =
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let input = Input::from(raw_input);
    let wires: Vec<Vec<LineSegment>> = input
        .split_lines(",")
        .unwrap()
        .iter()
        .map(|v| {
            let dirs = Directions::new(v.iter().map(|d| Direction::new(d)).collect());
            dirs.into()
        })
        .collect();
    let distance = part1(&wires).unwrap();
    assert_eq!(distance, 135);
}

/// Returns the point, and the Manhattan distance for the intersection of
/// any number of wires, that is closest to the origin.
fn part1(paths: &Vec<Vec<LineSegment>>) -> Result<i64> {
    let intersections = match find_intersections(paths) {
        Some(v) => v,
        None => {
            return Err(Error::new(ErrorKind::Other, "No intersections found."));
        }
    };

    // Find the intersection that is closest to the central point (0, 0).
    let origin = Point::new(0, 0);
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

        let origin = Point::new(0, 0);
        let next_wire = &wires[i + 1];
        for a in wire {
            for b in next_wire {
                if let Some(i) = a.intersection(&b) {
                    // Ignore the point if it's at (0, 0).
                    if i != origin {
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
    let input = Input::new_from_str("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap();
    let directions: Vec<Vec<Direction>> = input
        .split_lines(",")
        .unwrap()
        .iter()
        .map(|v| v.into_iter().map(|u| Direction::new(u)).collect())
        .collect();
    let steps = part2(&directions).unwrap();
    assert_eq!(steps, 30);

    let raw_input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let input = Input::new_from_str(raw_input).unwrap();
    let directions: Vec<Vec<Direction>> = input
        .split_lines(",")
        .unwrap()
        .iter()
        .map(|v| v.into_iter().map(|u| Direction::new(u)).collect())
        .collect();
    let steps = part2(&directions).unwrap();
    assert_eq!(steps, 610);

    let raw_input =
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let input = Input::new_from_str(raw_input).unwrap();
    let directions: Vec<Vec<Direction>> = input
        .split_lines(",")
        .unwrap()
        .iter()
        .map(|v| v.into_iter().map(|u| Direction::new(u)).collect())
        .collect();
    let steps = part2(&directions).unwrap();
    assert_eq!(steps, 410);
}

/// Find the intersections, but count the number of steps it took
/// for each wire to reach that intersection.
/// Sum the number of steps from each wire, and return the lowest sum.
fn part2(directions: &Vec<Vec<Direction>>) -> Result<i64> {
    let mut paths: Vec<Vec<LineSegment>> = vec![];
    for dirs in directions {
        let mut path: Vec<LineSegment> = vec![];
        let mut start = Point::new(0, 0);
        for dir in dirs {
            let end = start.travel(&dir);
            path.push(LineSegment::new(start, end));
            start = end;
        }
        paths.push(path);
    }
    let intersections = match find_intersections(&paths) {
        Some(v) => v,
        None => {
            return Err(Error::new(ErrorKind::Other, "No intersections found."));
        }
    };

    let mut steps_taken: HashMap<Point, Vec<i64>> = HashMap::new();
    for i in intersections {
        // The number of steps taken for each wire, to get to the
        // intersection `i`.
        let steps: Vec<i64> = directions.iter().map(|dir| walk_path(dir, &i)).collect();

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
    let mut start = Point::new(0, 0);
    for dir in directions {
        // Create a line segment for the current path we just took,
        // and check to see if the intersection is somewhere on that segment.
        let end = start.travel(dir);
        let ls = LineSegment::new(start, end);
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
