# advent-of-code

My solutions for the [Advent of Code](http://adventofcode.com) puzzles.
Some are in [Go](https://golang.org), and some are in
[Rust](https://rust-lang.org).
I also have some smatterings of other languages, like Lua
(for no reason other than wanting to have a little fun).

## Go solutions

All of the solutions written in Go are expecting to read input from STDIN.
So, to run them:

	$ go run 2016/01/part1.go < path/to/input.txt

substituting `path/to/input.txt` to the file holding your personalized input
data.

## Rust solutions

The Rust solutions expect to read puzzle input from a file,
whose path is specified as the first, positional argument to the program.
To run the Rust solutions:

	$ cd <YEAR>
	$ cargo run --bin <DD> -- path/to/input.txt

substituting `<YEAR>` with the event year (e.g. "2020"),
and substituting `<DD>` with the zero-prefixed day (e.g. "02").
