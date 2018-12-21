package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func main() {
	raw, err := ioutil.ReadFile("input.6")
	if err != nil {
		log.Fatalln("read file:", err)
	}

	input, err := parseInput(raw)
	if err != nil {
		log.Fatalln("parse input:", err)
	}

	test()

	a, b := run(input)
	fmt.Printf("p1=%d\np2=%d\n", a, b)
}

func parseInput(raw []byte) ([]int, error) {
	fields := strings.Fields(string(raw))

	input := make([]int, len(fields))
	for i, field := range fields {
		n, err := strconv.Atoi(field)
		if err != nil {
			return nil, err
		}
		input[i] = n
	}

	return input, nil
}

func test() {
	input, err := parseInput([]byte("0 2 7 0"))
	if err != nil {
		log.Fatalln(err)
	}

	a, b := run(input)

	// part 1
	if want, got := 5, a; want != got {
		log.Fatalf("wanted=%v, got=%v", want, got)
	}

	// part 2
	if want, got := 4, b; want != got {
		log.Fatalf("wanted=%d, got=%v", want, got)
	}
}

func run(input []int) (int, int) {
	var cycles [][]int
	cycle := make([]int, len(input))
	copy(cycle, input)
	for {
		fmt.Printf("\r%+v", cycle)

		// find the largest bank
		var max int
		for i := 0; i < len(cycle); i++ {
			if cycle[i] > cycle[max] {
				max = i
			}
		}

		//log.Printf("largestBank=%d", max)

		// redistribute the blocks
		val := cycle[max]
		cycle[max] = 0
		for i := max; i < len(cycle) && val > 0; val-- {
			// advance i, wrapping it around to 0 if necessary
			if i == len(cycle)-1 {
				i = 0
			} else {
				i++
			}

			//log.Printf("bank=%d val=%d", i, cycle[i]+1)
			cycle[i]++
		}

		// take a snapshot of the current memory bank layout
		c := make([]int, len(cycle))
		copy(c, cycle)
		cycles = append(cycles, c)

		// check for a duplicate snapshot
		var dup uint16
		for i := range cycles {
		Inner:
			for j := i + 1; j < len(cycles); j++ {
				for k := range cycles[i] {
					if cycles[i][k] == cycles[j][k] {
						dup |= 1 << uint(k)
					} else {
						continue Inner
					}
				}

				// exit if we have a match
				if dup & ^uint16(0) == dup {
					fmt.Printf("\n")
					return len(cycles), j - i
				}
			}
		}
	}
}
