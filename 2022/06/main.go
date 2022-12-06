package main

import (
	_ "embed"
	"fmt"
)

//go:embed input.txt
var input []byte

func main() {
	// Part 1: How many characters need to be processed before the first
	// start-of-packet marker is detected?
	fmt.Println(allUnique(input, 4))

	// Part 2: How many characters need to be processed before the first
	// start-of-message marker is detected?
	fmt.Println(allUnique(input, 14))
}

func allUnique(p []byte, n int) int {
	for i := 0; i < len(p)-n; i++ {
		m := make(map[byte]int)
		for j := i; j < i+n; j++ {
			count := m[input[j]] + 1
			m[input[j]] = count
		}

		if len(m) == n {
			// All different.
			return i + n
		}
	}
	return -1
}
