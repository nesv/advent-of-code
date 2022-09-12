package main

import (
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

//go:embed input/07
var input string

func main() {
	var crabs []int
	for _, v := range strings.Split(strings.TrimSpace(input), ",") {
		n, err := strconv.Atoi(v)
		if err != nil {
			fmt.Fprintln(os.Stderr, "parse number from input:", err)
			os.Exit(1)
		}
		crabs = append(crabs, n)
	}

	// Part 1: Determine the horizontal position that the crabs can align
	// to using the least fuel possible.
	// How much fuel must they spend to align to that position?
	fmt.Println(calculateLowestFuelUsage(crabs, part1))

	// Part 2: Same as part one, but instead of fuel being a constant
	// burn rate, the fuel consumption increases by 1 for each position
	// they have to move. (1 space = 1 fuel, 2 spaces = 3 fuel,
	// 3 spaces = 6 fuel, etc.).
	fmt.Println(calculateLowestFuelUsage(crabs, part2))
}

func calculateLowestFuelUsage(crabs []int, fn fuelCalcFunc) int {
	costs := make(map[int]int)
	for i := 0; i <= max(crabs); i++ {
		costs[i] = fn(crabs, i)
	}

	min := int(^uint(0) >> 1)
	for _, c := range costs {
		if c < min {
			min = c
		}
	}

	return min
}

func max(s []int) int {
	var n int
	for _, v := range s {
		if v > n {
			n = v
		}
	}
	return n
}

type fuelCalcFunc func(crabs []int, pos int) int

func part1(crabs []int, pos int) int {
	var fuel int
	for _, v := range crabs {
		cost := dist(v, pos)
		//fmt.Fprintf(os.Stderr, "part 1: cost %d -> %d = %d\n", v, pos, cost)
		fuel += cost
	}
	return fuel
}

func dist(start, end int) int {
	if start < end {
		return end - start
	}
	return start - end
}

func part2(crabs []int, pos int) int {
	var fuel int
	for _, v := range crabs {
		cost := triangular(dist(v, pos))
		//fmt.Fprintf(os.Stderr, "part 2: cost %d -> %d (%d) = %d\n", v, pos, dist(v, pos), cost)
		fuel += cost
	}
	return fuel
}

func triangular(n int) int {
	return ((n * n) + n) / 2
}
