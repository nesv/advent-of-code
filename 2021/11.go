// Day 11: Dumbo Octopus

package main

import (
	_ "embed"
	"fmt"
	"os"
	"unicode"
)

//go:embed input/11.test
var input []byte

func main() {
	var (
		grid [][]octopus
		row  []octopus
	)
	for _, b := range input {
		if unicode.IsSpace(rune(b)) {
			//fmt.Fprintf(os.Stderr, "%v\n", row)
			grid = append(grid, row)
			row = nil
			continue
		}
		row = append(row, octopus{energy: b - 48})
	}

	// Part 1: How many flashes have there been after 100 steps?
	fmt.Println(part1(grid))
}

func part1(octopuses [][]octopus) int {
	octos := make([][]octopus, len(octopuses))
	for _, row := range octopuses {
		o := make([]octopus, len(row))
		copy(o, row)
		octos = append(octos, o)
	}

	var flashes int
	for i := 0; i < 100; i++ {
		for y, row := range octos {
			for x, o := range row {
				o.energy++

				fmt.Fprintf(os.Stderr, "octopus %d,%d: %d\n", x, y, o.energy)

				if o.flashed {
					continue
				}

				if o.energy > 9 {
					fmt.Fprintf(os.Stderr, "octopus %d,%d flashed\n", x, y)
					o.flashed = true
					o.energy = 0
					flashes++
					ripple(octos, x, y)
				}
			}
		}

		reset(octos)
	}
	return flashes
}

func ripple(octos [][]octopus, x, y int) {

}

func reset(octopuses [][]octopus) {
	for _, row := range octopuses {
		for _, o := range row {
			o.flashed = false
		}
	}
}

type octopus struct {
	energy  uint8
	flashed bool
}
