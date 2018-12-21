package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("input.5")
	if err != nil {
		log.Fatalln(err)
	}
	defer f.Close()

	var input []int
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		i, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatalln("atoi:", err)
		}
		input = append(input, i)
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan:", err)
	}
	f.Close()

	test()

	in := make([]int, len(input))
	copy(in, input)
	fmt.Println(p1(in...))

	copy(in, input)
	fmt.Println(p2(in...))
}

func test() {
	input := []int{0, 3, 0, 1, -3}
	in := make([]int, len(input))
	var want int

	copy(in, input)
	log.Printf("%+v", in)
	want = 5
	if got := p1(in...); got != want {
		log.Fatalf("p1: wanted=%v, got=%v", want, got)
	}
	log.Printf("%+v", in)

	copy(in, input)
	log.Printf("%+v", in)
	want = 10
	if got := p2(in...); got != want {
		log.Fatalf("p2: wanted=%v, got=%v", want, got)
	}
	log.Printf("%+v", in)
}

// p1 returns the number of instructions that were processed until the
// instructions took us outside of the maze.
func p1(input ...int) int {
	// The number of steps we've taken.
	var n int

	// The current, and next instruction offsets.
	var cur, next int

	// The current instruction; start off at the first instruction.
	in := input[0]
	for {
		n++

		// Figure out where we are going to jump to, next.
		next = cur + in

		// Does it take us outside of the maze?
		if next < 0 || next >= len(input) {
			return n
		}

		// No?
		// Increment the number of steps we've taken, increment the
		// jump offset in the current instruction, and go to the next
		// offset.
		input[cur]++
		in, cur = input[next], next
	}

	return len(input)
}

func p2(input ...int) int {
	// The current, and next instruction offsets.
	var cur, next int

	// The current instruction; start off at the first instruction.
	in := input[0]
	for n := 1; ; n++ {
		// Figure out where we are going to jump to, next.
		next = cur + in

		// Does it take us outside of the maze?
		if next < 0 || next >= len(input) {
			return n
		}

		// No?
		if input[cur] >= 3 {
			input[cur]--
		} else {
			input[cur]++
		}
		in, cur = input[next], next
	}

	return len(input)
}
