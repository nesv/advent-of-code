package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"log"
	"sort"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	var (
		// The total calorie count per elf.
		// The is an optimization.
		calories []int

		scanner = bufio.NewScanner(strings.NewReader(input))
		current int
	)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			calories = append(calories, current)
			current = 0
			continue
		}

		n, err := strconv.Atoi(line)
		if err != nil {
			log.Fatalf("parse %q: %v\n", line, err)
		}
		current += n
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan input:", err)
	}

	// The value of [current] is not in [calories] slice yet,
	// so add it in.
	calories = append(calories, current)

	// Sort the elves so that the one with the fewest total calories is at the
	// start, and the one with the largest total calories is at the end.
	sort.Ints(calories)

	// Part 1: Find the elf carrying the most calories.
	// Return the number of calories that elf is carrying.
	n := len(calories) - 1
	fmt.Println(calories[n])

	// Part 2: Find the top three Elves carrying the most Calories.
	// How many Calories are those Elves carrying in total?
	fmt.Println(calories[n] + calories[n-1] + calories[n-2])
}
