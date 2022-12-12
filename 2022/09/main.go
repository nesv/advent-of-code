package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"image"
	"log"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	var (
		scanner      = bufio.NewScanner(strings.NewReader(input))
		instructions []instruction
	)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())

		n, err := strconv.Atoi(fields[1])
		if err != nil {
			log.Fatalln("atoi:", err)
		}

		instructions = append(instructions, instruction{
			direction: fields[0],
			steps:     n,
		})
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan input:", err)
	}

	fmt.Println(countTailLocations(2, instructions...))

	fmt.Println(countTailLocations(10, instructions...))
}

type instruction struct {
	direction string
	steps     int
}

func (i instruction) move(point image.Point) image.Point {
	var p image.Point
	switch i.direction {
	case "U":
		p = image.Point{Y: i.steps}
	case "D":
		p = image.Point{Y: -i.steps}
	case "L":
		p = image.Point{X: -i.steps}
	case "R":
		p = image.Point{X: i.steps}
	}
	return point.Add(p)
}

func (i instruction) vector() image.Point {
	switch i.direction {
	case "U":
		return image.Point{Y: 1}
	case "D":
		return image.Point{Y: -1}
	case "L":
		return image.Point{X: -1}
	}
	return image.Point{X: 1}
}

func (i instruction) String() string {
	return fmt.Sprintf("%s %d", i.direction, i.steps)
}

func countTailLocations(numKnots int, instructions ...instruction) int {
	var (
		knots = make([]image.Point, numKnots)
		seen  = make(map[image.Point]struct{})
	)
	// Add the starting location.
	seen[image.Point{X: 0, Y: 0}] = struct{}{}

	// Loop through all of the instructions.
	for _, inst := range instructions {
		log.Println(inst)
		head := inst.move(knots[0])
		log.Printf("head: %s -> %s\n", knots[0], head)
		knots[0] = head

		// For each of the remaining knots,
		// make sure they are all touching the previous knot.
		for k := 1; k < len(knots); k++ {
			head, tail := knots[k-1], knots[k]

			// If the head is covering the tail, just go on to the next instruction.
			if head.Eq(tail) {
				log.Println("head covering tail")
				continue
			}

			for !tailIsTouching(head, tail) {
				newtail := tail.Add(nextTailMove(head, tail))
				log.Printf("knot %d: %s -> %s\n", k, tail, newtail)
				tail = newtail
				knots[k] = tail

				if k == numKnots-1 {
					seen[tail] = struct{}{}
				}
			}
		}
	}

	return len(seen)
}

func tailIsTouching(head, tail image.Point) bool {
	sub := head.Sub(tail)
	return abs(sub.X) < 2 && abs(sub.Y) < 2
}

func abs(n int) int {
	if n == 0 {
		return 0
	}
	if n < 0 {
		return n * -1
	}
	return n
}

func nextTailMove(head, tail image.Point) image.Point {
	sub := head.Sub(tail)
	x, y := sub.X, sub.Y

	if x != 0 {
		if x < 0 {
			x /= -x
		} else {
			x /= x
		}
	}

	if y != 0 {
		if y < 0 {
			y /= -y
		} else {
			y /= y
		}
	}

	return image.Point{X: x, Y: y}
}
