package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

//go:embed input/02
var input string

func main() {
	var (
		directions []direction
		scanner    = bufio.NewScanner(strings.NewReader(input))
	)
	for scanner.Scan() {
		if scanner.Text() == "" {
			continue
		}

		fields := strings.Fields(scanner.Text())
		if len(fields) != 2 {
			fmt.Fprintln(os.Stderr, "bad input: %q", scanner.Text())
			continue
		}
		switch fields[0] {
		case "forward", "down", "up":
			n, err := strconv.Atoi(fields[1])
			if err != nil {
				fmt.Fprintln(os.Stderr, "parse %q: %s", fields[1], err)
				continue
			}
			directions = append(directions, direction{
				dir: fields[0],
				n:   n,
			})
		default:
			fmt.Fprintln(os.Stderr, "bad input: %q", scanner.Text())
		}
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	// Part 1: Follow the given directions.
	fmt.Println(follow(directions))

	// Part 2: Follow the given directions, but with "aim".
	fmt.Println(followWithAim(directions))
}

type direction struct {
	dir string
	n   int
}

func follow(directions []direction) (position int) {
	var depth, hpos int
	for _, d := range directions {
		switch d.dir {
		case "forward":
			hpos += d.n
		case "up":
			depth -= d.n
		case "down":
			depth += d.n
		}
	}
	return depth * hpos
}

func followWithAim(directions []direction) (position int) {
	var depth, hpos, aim int
	for _, d := range directions {
		switch d.dir {
		case "forward":
			hpos += d.n
			depth += d.n * aim
		case "up":
			aim -= d.n
		case "down":
			aim += d.n
		}
	}
	return depth * hpos
}
