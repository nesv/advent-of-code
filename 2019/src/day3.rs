use std::fmt;
use std::iter::{FromIterator, IntoIterator, Iterator};

#[derive(Clone)]
pub struct Directions(Vec<Direction>);

impl Directions {
    pub fn new(v: Vec<Direction>) -> Self {
        Directions(v)
    }

    /// Travels the distance of the given path, and counts the number of steps
    /// taken until it reaches the given intersection.
    pub fn walk_path(&self, intersection: &Point) -> i64 {
        let mut steps: i64 = 0;
        let mut start = Point::new(0, 0);
        for dir in self.0.iter() {
            // Create a line segment for the current path we just took,
            // and check to see if the intersection is somewhere on that segment.
            let end = start.travel(&dir);
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
}

impl Into<Vec<LineSegment>> for Directions {
    fn into(self) -> Vec<LineSegment> {
        let mut segments: Vec<LineSegment> = vec![];
        let mut start = Point::new(0, 0);
        for dir in self.0 {
            let end = start.travel(&dir);
            segments.push(LineSegment::new(start, end));
            start = end;
        }
        segments
    }
}

impl Into<Vec<Direction>> for Directions {
    fn into(self) -> Vec<Direction> {
        self.0
    }
}

impl From<Vec<Direction>> for Directions {
    fn from(dirs: Vec<Direction>) -> Self {
        Self::new(dirs)
    }
}

impl FromIterator<Direction> for Directions {
    fn from_iter<I: IntoIterator<Item = Direction>>(iter: I) -> Self {
        let mut c = Vec::new();
        for i in iter {
            c.push(i);
        }
        Directions::from(c)
    }
}

/// Direction represents a direction given from the puzzle input.
///
/// #Example
///
/// ```
/// let d = Direction::new("R35");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Unknown(String),
    Right(i64),
    Left(i64),
    Up(i64),
    Down(i64),
}

impl Direction {
    /// Creates a new Direction, from a string like `"R35"`.
    ///
    /// If the first character of the direction string does not being with one
    /// of `D`, `L`, `R`, or `U`, new will return a `Direction::Unknown` with
    /// the value being the malformed input `s`.
    pub fn new(s: &str) -> Self {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point(i64, i64);

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point(x, y)
    }

    /// Distance calculates the Manhattan distance to the other Point.
    pub fn distance_from(&self, p: &Point) -> i64 {
        let dx = self.0 - p.0;
        let dy = self.1 - p.1;
        dx.abs() + dy.abs()
    }

    // Indicates whether this Point is on the given line segment, `s`.
    pub fn on_segment(&self, s: &LineSegment) -> bool {
        use crate::util::{max, min};
        (self.0 <= max(s.start.0, s.end.0) && self.0 >= min(s.start.0, s.end.0))
            && (self.1 <= max(s.start.1, s.end.1) && self.1 >= min(s.start.1, s.end.1))
    }

    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }

    /// Travel returns a new `Point` that follows the given `Direction`,
    /// starting at the current point.
    pub fn travel(&self, to: &Direction) -> Self {
        match to {
            Direction::Right(n) => Point::new(self.x() + n, self.y()),
            Direction::Left(n) => Point::new(self.x() - n, self.y()),
            Direction::Up(n) => Point::new(self.x(), self.y() + n),
            Direction::Down(n) => Point::new(self.x(), self.y() - n),
            Direction::Unknown(_) => Point::new(0, 0),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    /// Returns a new line segment from the given `start` and `end` points.
    pub fn new(start: Point, end: Point) -> Self {
        LineSegment { start, end }
    }

    /// Returns the point of intersection between a this line segment, and
    /// another.
    pub fn intersection(&self, other: &LineSegment) -> Option<Point> {
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

    /// Returns the beginning and ending points of the line segment.
    pub fn points(&self) -> (Point, Point) {
        (self.start.clone(), self.end.clone())
    }
}

#[derive(PartialEq)]
enum Orientation {
    Colinear,
    Clockwise,
    CounterClockwise,
}
