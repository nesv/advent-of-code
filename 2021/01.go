// Day 1: Sonar Sweep
package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

//go:embed input/01
var input string

func main() {
	var (
		nums    []int
		scanner = bufio.NewScanner(strings.NewReader(input))
	)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		n, err := strconv.Atoi(line)
		if err != nil {
			fmt.Fprintln(os.Stderr, "parse input %q: %s", line, err)
			os.Exit(1)
		}
		nums = append(nums, n)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "error: %s", err)
		os.Exit(1)
	}

	// Part 1: Count the number of times a depth measurement increases
	// from the previous measurement.
	fmt.Println(depthIncreases(nums))

	// Part 2: Count the number of times the sum of measurements in a
	// sliding window increases.
	fmt.Println(windowedDepthIncreases(nums))
}

func depthIncreases(depths []int) int {
	var n int
	for i := 1; i < len(depths); i++ {
		if depths[i] > depths[i-1] {
			n++
		}
	}
	return n
}

func windowedDepthIncreases(depths []int) int {
	var (
		prev int
		n    int
	)
	for i := 0; i < len(depths)-3; i++ {
		sum := depths[i] + depths[i+1] + depths[i+2]
		if sum > prev {
			n++
		}
		prev = sum
	}
	return n
}
