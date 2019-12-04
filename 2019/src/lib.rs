mod day3;
mod input;

pub use day3::{Direction, Directions, LineSegment, Point};
pub use input::Input;

/// The `util` module provides some convenience utilities that are not tied
/// to a specific puzzle.
pub mod util {
    /// Return the larger of `a` or `b`.
    pub fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            return a;
        }
        b
    }

    /// Return the smaller of `a` or `b`.
    pub fn min<T: PartialOrd>(a: T, b: T) -> T {
        if a < b {
            return a;
        }
        b
    }
}