package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

//go:embed input/03
var input string

func main() {
	var (
		readings []uint
		scanner  = bufio.NewScanner(strings.NewReader(input))
	)
	for scanner.Scan() {
		if scanner.Text() == "" {
			continue
		}

		n, err := strconv.ParseUint(scanner.Text(), 2, 16)
		if err != nil {
			fmt.Fprintf(os.Stderr, "parse error: %q: %s\n", scanner.Text(), err)
			os.Exit(1)
		}

		readings = append(readings, uint(n))
	}

	// Part 1: Power consumption.
	fmt.Println(powerConsumption(readings))

	// Part 2: Life support rating.
	fmt.Println(lifeSupportRating(readings))
}

func powerConsumption(readings []uint) uint {
	// Calculate the "gamma rate".
	var gamma uint
	for i := 0; i < 12; i++ {
		var (
			mask = uint(1 << i)
			ones int
		)
		for _, r := range readings {
			if r&mask == mask {
				ones++
			}
		}
		if ones >= len(readings)/2 {
			gamma |= mask
		}
	}

	// Calculate the "epsilon rate".
	epsilon := ^gamma & 0b111111111111

	// Return the power consumption.
	return gamma * epsilon
}

func lifeSupportRating(readings []uint) uint {
	return oxygenRating(readings) * co2ScrubberRating(readings)
}

func oxygenRating(readings []uint) uint {
	rs := make([]uint, len(readings))
	copy(rs, readings)

	for i := 11; i >= 0; i-- {
		mask := uint(1 << i)
		split := partition(rs, func(n uint) bool { return n&mask == mask })
		if split >= len(rs)/2 {
			rs = rs[:split]
		} else {
			rs = rs[split:]
		}
		if len(rs) == 1 {
			break
		}
	}
	return rs[0]
}

// partition sorts readings so that elements for which the predicate function
// return true, are earlier in the slice.
// partition returns the index for the first value that does not match the
// predicate.
// This function does not maintain the ordering of elements after the split.
func partition(readings []uint, predicate func(uint) bool) int {
	split := 0
	for i := 0; i < len(readings); i++ {
		if predicate(readings[i]) {
			readings[split], readings[i] = readings[i], readings[split]
			split++
		}
	}
	return split
}

func co2ScrubberRating(readings []uint) uint {
	rs := make([]uint, len(readings))
	copy(rs, readings)

	for i := 11; i >= 0; i-- {
		mask := uint(1 << i)
		split := partition(rs, func(n uint) bool { return n&mask == mask })
		if split < len(rs)/2 {
			rs = rs[:split]
		} else {
			rs = rs[split:]
		}
		if len(rs) == 1 {
			break
		}
	}
	return rs[0]
}
