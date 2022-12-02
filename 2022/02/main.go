package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"log"
	"strings"
)

//go:embed input.txt
var input string

// Scoring for the chosen shape.
const (
	rock     = 1
	paper    = 2
	scissors = 3
)

// Scoring for whether the round was won, lost, or ended in a draw.
const (
	lose = 0
	draw = 3
	win  = 6
)

func main() {
	var (
		rounds []round

		r       = strings.NewReader(input)
		scanner = bufio.NewScanner(r)
	)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		rounds = append(rounds, round{
			opponent: fields[0],
			you:      fields[1],
		})
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan input:", err)
	}

	// What would your total score be if everything goes exactly according
	// to your strategy guide?
	var score int
	for _, r := range rounds {
		score += r.part1()
	}
	fmt.Println(score)

	// Following the Elf's instructions for the second column, what would
	// your total score be if everything goes exactly according to your
	// strategy guide?
	score = 0
	for _, r := range rounds {
		score += r.part2()
	}
	fmt.Println(score)
}

type round struct {
	opponent string
	you      string
}

// part1 returns the score for the round, according to the (assumed) rules
// for the first part of the puzzle.
//
// Here, you assume that:
//
//	"A", "X" == rock
//	"B", "Y" == paper
//	"C", "Z" == scissors
func (r round) part1() int {
	switch r.opponent {
	case "A":
		switch r.you {
		case "X":
			return rock + draw
		case "Y":
			return paper + win
		case "Z":
			return scissors + lose
		}
	case "B":
		switch r.you {
		case "X":
			return rock + lose
		case "Y":
			return paper + draw
		case "Z":
			return scissors + win
		}
	case "C":
		switch r.you {
		case "X":
			return rock + win
		case "Y":
			return paper + lose
		case "Z":
			return scissors + draw
		}
	}

	return 0
}

// part2 returns the score for the round, according to the instruction from
// the elf.
//
// Here:
//
//	"X" = lose
//	"Y" = draw
//	"Z" = win
func (r round) part2() int {
	switch r.you {
	case "X":
		// Lose.
		switch r.opponent {
		case "A":
			return lose + scissors
		case "B":
			return lose + rock
		case "C":
			return lose + paper
		}
	case "Y":
		// Draw.
		switch r.opponent {
		case "A":
			return draw + rock
		case "B":
			return draw + paper
		case "C":
			return draw + scissors
		}
	case "Z":
		// Win.
		switch r.opponent {
		case "A":
			return win + paper
		case "B":
			return win + scissors
		case "C":
			return win + rock
		}
	}

	return 0
}
