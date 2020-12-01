//! Helper types and functions for
//! [Advent of Code 2020](https://adventofcode.com/2020).

mod input;

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
