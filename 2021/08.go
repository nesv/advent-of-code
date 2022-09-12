package main

// Day 8: Seven Segment Search

import (
	"bufio"
	_ "embed"
	"fmt"
	"math/bits"
	"os"
	"strings"
)

//go:embed input/08.test
var input string

func main() {
	var (
		signals [][]string
		outputs [][]string

		scanner = bufio.NewScanner(strings.NewReader(input))
	)
	for scanner.Scan() {
		parts := strings.Split(scanner.Text(), "|")

		s := strings.Fields(parts[0])
		signals = append(signals, s)

		o := strings.Fields(parts[1])
		outputs = append(outputs, o)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "scan error:", err)
		os.Exit(1)
	}

	// Part 1: In the output values, how many times do the digits
	// 1, 4, 7, or 8 appear?
	fmt.Println(part1(outputs))

	// Part 2: What do you get if you add up all of the output values?
	// The puzzle instructions explain the mapping of signals to their
	// position on the seven segment display.
	fmt.Println(part2(signals, outputs))

	println("pow10(0) =", pow10(0))
	println("pow10(1) =", pow10(1))
	println("pow10(2) =", pow10(2))
	println("pow10(3) =", pow10(3))
	println("pow10(4) =", pow10(4))
}

func part1(outputs [][]string) int {
	var n int
	for _, o := range outputs {
		for _, v := range o {
			switch len(v) {
			case 2, 4, 3, 7:
				// Digits: 1, 4, 7, 8.
				n++
			}
		}
	}
	return n
}

func part2(signals, outputs [][]string) int {
	var sum int
	for i := 0; i < len(signals); i++ {
		segments := learnSignals(signals[i])
		var dv []int // debug
		var s int
		for j, o := range outputs[i] {
			sv := segments[segval(o)]
			dv = append(dv, sv)

			if j == len(outputs[i])-1 {
				s += sv
			} else {
				m := pow10(len(outputs[i]) - j - 1)
				s += sv * m
			}
		}
		sum += s
		fmt.Fprintln(os.Stderr, outputs[i], "=>", dv, "=>", s)
	}
	return sum
}

func pow10(n int) int {
	p := 1
	for i := 0; i < n; i++ {
		p *= 10
	}
	return p
}

func learnSignals(signals []string) map[uint]int {
	segments := make(map[uint]int)
	rev := make(map[int]uint)

	// Do one pass to get all of the easy digits, first.
	for _, s := range signals {
		v := segval(s)
		switch len(s) {
		case 2:
			segments[v] = 1
			rev[1] = v
		case 3:
			segments[v] = 7
			rev[7] = v
		case 4:
			segments[v] = 4
			rev[4] = v
		case 7:
			segments[v] = 8
			rev[8] = v
		}
	}

	// Do a second pass to get all of the remaining digits.
	for _, s := range signals {
		switch len(s) {
		case 2, 3, 4, 7:
			// 1,4,7,8
			continue
		case 5:
			// 2,3,5
			v := segval(s)
			switch ones := bits.OnesCount(v ^ (rev[7] | rev[4])); ones {
			case 2:
				segments[v] = 5
			case 1:
				segments[v] = 3
			case 4:
				segments[v] = 2
			default:
				panic(fmt.Sprintf("%s => %d", s, ones))
			}

		case 6:
			// 0,6,9
			v := segval(s)
			switch ones := bits.OnesCount(v ^ rev[4]); ones {
			case 2:
				segments[v] = 9
			case 3:
				segments[v] = 0
			case 4:
				segments[v] = 6
			default:
				panic(fmt.Sprintf("%q => %d", s, ones))
			}
		}
	}

	return segments
}

func segval(s string) uint {
	var v uint
	for _, c := range s {
		switch c {
		case 'a':
			v |= sa
		case 'b':
			v |= sb
		case 'c':
			v |= sc
		case 'd':
			v |= sd
		case 'e':
			v |= se
		case 'f':
			v |= sf
		case 'g':
			v |= sg
		}
	}
	return v
}

const (
	sa uint = 1 << iota
	sb
	sc
	sd
	se
	sf
	sg
)
