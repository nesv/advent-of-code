package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"io"
	"log"
	"strings"
	"unicode"
)

//go:embed input.txt
var input string

func main() {
	// Part 1: After the rearrangement procedure completes,
	// what crate ends up on top of each stack?
	stacks, instructions, err := parseInput(strings.NewReader(input))
	if err != nil {
		log.Fatalln(err)
	}

	for _, in := range instructions {
		in.do(stacks)
	}

	var tops []byte
	for i := 1; i <= len(stacks); i++ {
		tops = append(tops, stacks[i].peek())
	}
	fmt.Println(string(tops))

	// Part 2: The same as part 1 (which crate ends up on the top of each stack)
	// but the arrangement method is a little different.
	stacks, instructions, err = parseInput(strings.NewReader(input))
	if err != nil {
		log.Fatalln(err)
	}
	for _, in := range instructions {
		in.do2(stacks)
	}

	tops = nil
	for i := 1; i <= len(stacks); i++ {
		tops = append(tops, stacks[i].peek())
	}
	fmt.Println(string(tops))
}

func parseInput(r io.Reader) (stacks map[int]*stack, instructions []instruction, err error) {
	var (
		scanner          = bufio.NewScanner(r)
		numStacks        int
		readInstructions bool
	)
	for scanner.Scan() {
		if readInstructions {
			var n, from, to int
			if _, err := fmt.Sscanf(scanner.Text(), "move %d from %d to %d\n", &n, &from, &to); err != nil {
				return nil, nil, fmt.Errorf("scan line: %w", err)
			}

			instructions = append(instructions, instruction{
				n:    n,
				from: from,
				to:   to,
			})

			continue
		}

		// If this is an empty line once we have trimmed all the whitespace,
		// this is the line that ends the initial stack layout.
		// The rest of the lines will be move instructions.
		if scanner.Text() == "" {
			readInstructions = true
			continue
		}

		line := []byte(scanner.Text())

		// If the byte at index 1 is the '1' character,
		// this is the line underneath the current stack allocations.
		// Skip it.
		if line[1] == '1' {
			continue
		}

		// Allocate the number of stacks based on the length of the line.
		if numStacks == 0 {
			numStacks = (len(line) + 1) / 4
			stacks = make(map[int]*stack, numStacks)
			for i := 0; i < numStacks; i++ {
				stacks[i+1] = &stack{
					values: []byte{},
				}
			}
		}

		// Read the character/byte in each stack.
		// A whitespace character means there is no item at that height in the stack.
		stackNum := 1
		for i := 1; i < len(line); i += 4 {
			if unicode.IsSpace(rune(line[i])) {
				stackNum++
				continue
			}
			stacks[stackNum].shift(line[i])
			stackNum++
		}
	}
	if err := scanner.Err(); err != nil {
		return nil, nil, fmt.Errorf("scan input: %w", err)
	}

	return stacks, instructions, nil
}

type stack struct {
	values []byte
}

// pop removes a byte from the end of the stack.
func (s *stack) pop() byte {
	if len(s.values) == 0 {
		return 0
	}

	n := len(s.values) - 1
	b := s.values[n]
	s.values = s.values[:n]
	return b
}

// push adds a byte to the end of the stack.
func (s *stack) push(b byte) {
	s.values = append(s.values, b)
}

// shift adds a byte to the beginning of the stack.
func (s *stack) shift(b byte) {
	s.values = append([]byte{b}, s.values...)
}

// peek shows the byte that would be returned by pop,
// without removing it.
func (s *stack) peek() byte {
	if n := len(s.values); n != 0 {
		return s.values[n-1]
	}
	return 0
}

// take removes the top-most n elements off stack,
// and maintains their order.
func (s *stack) take(n int) []byte {
	var p []byte
	for i := 0; i < n; i++ {
		p = append([]byte{s.pop()}, p...)
	}
	return p
}

func (s stack) String() string {
	var b strings.Builder
	for _, v := range s.values {
		b.WriteString(fmt.Sprintf("[%c]", v))
	}
	return b.String()
}

type instruction struct {
	n    int // number of items to move
	from int // from stack
	to   int // to stack
}

func (i instruction) String() string {
	return fmt.Sprintf("move %d from %d to %d", i.n, i.from, i.to)
}

func (i instruction) do(stacks map[int]*stack) {
	for n := 0; n < i.n; n++ {
		v := stacks[i.from].pop()
		stacks[i.to].push(v)
	}
}

func (i instruction) do2(stacks map[int]*stack) {
	crates := stacks[i.from].take(i.n)
	for _, c := range crates {
		stacks[i.to].push(c)
	}
}
