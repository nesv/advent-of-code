# advent-of-code

My solutions for the [Advent of Code][advent-of-code] puzzles.
Some are in [Go][go], and some are in [Rust][rust].
I also have some smatterings of other languages, like [Lua][lua]
(for no reason other than wanting to have a little fun).

[advent-of-code]: https://adventofcode.com
[go]: https://golang.org
[rust]: https://rust-lang.org
[lua]: https://www.lua.org/

## Go solutions

All of the solutions written in Go are expecting to read input from STDIN.
So, to run them:

    $ go run 2016/01/part1.go < path/to/input.txt

substituting `path/to/input.txt` to the file holding your personalized input
data.

### Go: 2022

The 2022 solutions use the [embed][go-embed] package to embed the puzzle input
into the executable at compile time.

[go-embed]: https://pkg.go.dev/embed

To run the 2022 solutions:

    $ go run 2022/01/main.go

## Rust solutions

The Rust solutions expect to read puzzle input from a file,
whose path is specified as the first, positional argument to the program.
To run the Rust solutions:

	$ cd <YEAR>
	$ cargo run --bin <DD> -- path/to/input.txt

substituting `<YEAR>` with the event year (e.g. "2020"),
and substituting `<DD>` with the zero-prefixed day (e.g. "02").
