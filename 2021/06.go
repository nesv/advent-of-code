package main

import (
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

//go:embed input/06
var input string

func main() {
	var fish []int
	for _, v := range strings.Split(strings.TrimSpace(input), ",") {
		n, err := strconv.Atoi(v)
		if err != nil {
			fmt.Fprintln(os.Stderr, "parse number from input:", err)
			os.Exit(1)
		}
		fish = append(fish, n)
	}

	// Part 1: How many lanternfish would exist after 80 days?
	fmt.Println(countLanternfish(fish, 80))

	// Part 2: How many lanternfish would exist after 256 days?
	fmt.Println(countLanternfish(fish, 256))
}

func countLanternfish(fish []int, days int) int {
	// Seed the initial state.
	state := make(map[int]int)
	for i := 0; i < len(fish); i++ {
		state[fish[i]]++
	}

	for d := 0; d < days; d++ {
		// Capture the number of fish who are about to reproduce.
		aboutToReproduce := state[0]

		// Shuffle the number of fish in any given lifetime into the
		// current day.
		for i := 0; i < 8; i++ {
			state[i] = state[i+1]
		}

		// Once fish reproduce, their "timers" get reset to 6 days.
		state[6] += aboutToReproduce

		// The number of fish that were just born.
		state[8] = aboutToReproduce
	}

	var sum int
	for _, v := range state {
		sum += v
	}
	return sum
}
