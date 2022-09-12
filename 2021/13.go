// Day 13: Transparent Origami.

package main

import (
	"bufio"
	"bytes"
	_ "embed"
	"fmt"
	"log"
	"strconv"
	"strings"
)

//go:embed input/13
var input []byte

func main() {
	var (
		points  []point
		scanner = bufio.NewScanner(bytes.NewReader(input))
	)
	// For the first pass, scan the coordinates, and stop at the empty line.
	for scanner.Scan() {
		// Stop when we hit the empty line.
		if scanner.Text() == "" {
			break
		}

		fields := strings.Split(scanner.Text(), ",")

		x, err := strconv.Atoi(fields[0])
		if err != nil {
			log.Fatalf("atoi %q: %v", fields[0], err)
		}

		y, err := strconv.Atoi(fields[1])
		if err != nil {
			log.Fatalf("atoi %q: %v", fields[1], err)
		}

		points = append(points, point{x: x, y: y})
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan error:", err)
	}

	// Now, continue scanning, but this time, they will be fold
	// instructions.
	var folds []foldinst
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		sep := strings.Index(fields[2],"=")
		if sep == -1 {
			log.Fatalln("bad input, no separator: %q",scanner.Text())
		}

		v, err := strconv.Atoi(fields[2][sep+1:])
		if err != nil {
			log.Fatalf("atoi %q: %v",fields[2][sep+1:], err)
		}
		switch fields[2][:sep] {
		case "x":
			folds=append(folds,foldinst{x:v})
		case "y":
			folds=append(folds,foldinst{y:v})
		default:
			log.Fatalf("bad fold instruction: %q", scanner.Text())
		}
	}
	if err := scanner.Err();err!=nil{
		log.Fatalln("scan error:",err)
	}

	// Part 1: How many dots are visible after completing just the first
	// fold instruction on your transparent paper?
	grid := fold(points,folds,1)
	fmt.Println(fold(points, folds, 1))
}

type point struct {
	x, y int
}

func (p *point) foldx(v int) {}

func (p *point) foldy(v int) {}

// foldinst represents a fold instruction.
// Only x or y will be set, never both.
type foldinst struct {
	x, y int
}

func fold(points []point, folds []foldinst, n int) map[int]map[int]bool {
	m := make(map[int]map[int]bool)
	for _, p := range points {
}
