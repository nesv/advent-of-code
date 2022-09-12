// Day 10: Syntax Scoring

package main

import (
	"bytes"
	_ "embed"
	"fmt"
	"sort"
)

//go:embed input/10
var input []byte

func main() {
	lines := bytes.Fields(input)

	// Sort the lines.
	var (
		incomplete [][]byte
		corrupted  [][]byte
		badChars   []byte
	)
	for _, line := range lines {
		inc, cor, c := check(line)
		if inc {
			incomplete = append(incomplete, line)
		} else if cor {
			corrupted = append(corrupted, line)
			badChars = append(badChars, c)
		}
	}

	// Part 1: What is the total syntax error score for the corrupted lines?
	fmt.Println(syntaxErrorScore(badChars))

	// Part 2: What is the middle score?
	var scores []int
	for _, line := range incomplete {
		scores = append(scores, incompleteLineScore(line))
	}
	sort.Ints(scores)
	fmt.Println(scores[len(scores)/2])
}

func check(line []byte) (incomplete, corrupted bool, bad byte) {
	var (
		expect []byte
		n      int
	)
	for i, c := range line {
		switch line[i] {
		case '(':
			expect = append(expect, ')')
			n++
			continue
		case '{':
			expect = append(expect, '}')
			n++
			continue
		case '[':
			expect = append(expect, ']')
			n++
			continue
		case '<':
			expect = append(expect, '>')
			n++
			continue
		}

		if want := expect[len(expect)-1]; c != want {
			return false, true, c
		}
		expect = expect[:len(expect)-1]
		n--
	}
	if n > 0 {
		return true, false, 0
	}
	return false, false, 0
}

func syntaxErrorScore(chars []byte) int {
	var score int
	for _, c := range chars {
		switch c {
		case ')':
			score += 3
		case ']':
			score += 57
		case '}':
			score += 1197
		case '>':
			score += 25137
		}
	}
	return score
}

func incompleteLineScore(line []byte) int {
	var remaining []byte
	for _, c := range line {
		switch c {
		case '(':
			remaining = append(remaining, ')')
			continue
		case '[':
			remaining = append(remaining, ']')
			continue
		case '{':
			remaining = append(remaining, '}')
			continue
		case '<':
			remaining = append(remaining, '>')
			continue
		}

		if want := remaining[len(remaining)-1]; want != c {
			panic("corrupt line")
		}
		remaining = remaining[:len(remaining)-1]
	}

	var score int
	for i := len(remaining) - 1; i >= 0; i-- {
		score *= 5
		switch remaining[i] {
		case ')':
			score += 1
		case ']':
			score += 2
		case '}':
			score += 3
		case '>':
			score += 4
		}
	}
	return score
}
