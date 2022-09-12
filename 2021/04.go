package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

//go:embed input/04
var input string

func main() {
	// The first line of the input is the numbers that are called.
	// The remainder of the input is the boards.
	// Each board is separated by an empty line, including the first board,
	// which is separated from the numbers that get called, by an empty line.
	var (
		numbers []int
		boards  []board
		scanner = bufio.NewScanner(strings.NewReader(input))

		currentBoard board
	)
	for scanner.Scan() {
		if len(numbers) == 0 {
			// Parse the numbers that will be called out.
			nn := strings.Split(scanner.Text(), ",")
			for _, n := range nn {
				v, err := strconv.Atoi(n)
				if err != nil {
					fmt.Fprintf(os.Stderr, "parse error: %q: %s\n", n, err)
					os.Exit(1)
				}
				numbers = append(numbers, v)
			}

			// Consume the next empty line (safely).
			if !scanner.Scan() {
				break
			}

			continue
		}

		if scanner.Text() == "" {
			// The current board is complete.
			boards = append(boards, currentBoard)
			currentBoard = board{}
			continue
		}

		var row []int
		fields := strings.Fields(scanner.Text())
		for _, v := range fields {
			n, err := strconv.Atoi(v)
			if err != nil {
				fmt.Fprintf(os.Stderr, "parse error: %q: %s\n", v, err)
				os.Exit(1)
			}
			row = append(row, n)
		}
		currentBoard.rows = append(currentBoard.rows, row)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "scan error:", err)
		os.Exit(1)
	}
	if len(currentBoard.rows) > 0 {
		boards = append(boards, currentBoard)
	}

	fmt.Fprintln(os.Stderr, "numbers =", numbers)

	// Part 1: Find the board that will win first.
	fmt.Println(winningBoardScore(numbers, boards))

	// Part 2: Find the board that will win last.
	fmt.Println(lastWinningBoardScore(numbers, boards))
}

type board struct {
	rows [][]int
}

// mark scans the board to see if n is present anywhere,
// and marks it as -1 if it is found.
func (b *board) mark(n int) {
	for i, r := range b.rows {
		for j, v := range r {
			if v == n {
				b.rows[i][j] = -1
				return
			}
		}
	}
}

func (b board) won() bool {
	// Check to see if there are any rows with all -1.
	for i := 0; i < len(b.rows); i++ {
		rowp, colp := true, true
		for j := 0; j < len(b.rows[i]); j++ {
			if b.rows[i][j] != -1 {
				rowp = false
			}
			if b.rows[j][i] != -1 {
				colp = false
			}
		}
		if rowp || colp {
			return true
		}
	}
	return false
}

func (b board) score() int {
	var sum int
	for _, row := range b.rows {
		for _, v := range row {
			if v > -1 {
				sum += v
			}
		}
	}
	return sum
}

func (b board) String() string {
	var s strings.Builder
	for _, row := range b.rows {
		for _, v := range row {
			fmt.Fprintf(&s, "%2.d ", v)
		}
		fmt.Fprint(&s, "\n")
	}
	return s.String()
}

func winningBoardScore(numbers []int, boards []board) int {
	for _, n := range numbers {
		for _, b := range boards {
			b.mark(n)
			if b.won() {
				return b.score() * n
			}
		}
	}
	return 0
}

func lastWinningBoardScore(numbers []int, boards []board) int {
	var (
		won   = make(map[int]struct{})
		score int
	)
	for _, n := range numbers {
		for i, b := range boards {
			if _, alreadyWon := won[i]; alreadyWon {
				continue
			}

			b.mark(n)
			if b.won() {
				won[i] = struct{}{}
				score = b.score() * n
			}
		}
	}
	return score
}
