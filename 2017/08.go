package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
	f, err := os.Open("input.8")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	test()

	input, err := parseInput(f)
	if err != nil {
		panic(err)
	}

	fmt.Println(p1(input))
	fmt.Println(p2(input))
}

func test() {
	raw := []byte(`b inc 5 if a > 1
	a inc 1 if b < 5
	c dec -10 if a >= 1
	c inc -20 if c == 10`)
	input, err := parseInput(bytes.NewReader(raw))
	if err != nil {
		log.Fatalln(err)
	}

	if want, got := 1, p1(input); want != got {
		log.Fatalf("part 1: wanted=%v got=%v", want, got)
	}

	if want, got := 10, p2(input); want != got {
		log.Fatalf("part 2: wanted=%v got=%v", want, got)
	}
}

func parseInput(r io.Reader) ([]instruction, error) {
	var ins []instruction
	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		var reg, op, treg, cond string
		var val, tval int

		if _, err := fmt.Sscanf(scanner.Text(), "%s %s %d if %s %s %d",
			&reg,
			&op,
			&val,
			&treg,
			&cond,
			&tval,
		); err != nil {
			return nil, err
		}

		ins = append(ins, instruction{
			reg:  reg,
			op:   op,
			val:  val,
			treg: treg,
			cond: getConditionFromString(cond),
			tval: tval,
		})

	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return ins, nil
}

type instruction struct {
	reg  string
	op   string
	val  int
	treg string
	cond condition
	tval int
}

type condition func(a, b int) bool

var (
	gt condition = func(a, b int) bool {
		return a > b
	}

	lt condition = func(a, b int) bool {
		return a < b
	}

	eq condition = func(a, b int) bool {
		return a == b
	}

	ne condition = func(a, b int) bool {
		return a != b
	}

	ge condition = func(a, b int) bool {
		return a >= b
	}

	le condition = func(a, b int) bool {
		return a <= b
	}
)

func getConditionFromString(condStr string) condition {
	switch condStr {
	case ">":
		return gt
	case "<":
		return lt
	case "==":
		return eq
	case "!=":
		return ne
	case ">=":
		return ge
	case "<=":
		return le
	}
	panic(fmt.Sprintf("unexpected condition string %q", condStr))
}

func p1(input []instruction) int {
	registers := make(map[string]int)
	for _, in := range input {
		if _, ok := registers[in.reg]; !ok {
			registers[in.reg] = 0
		}
		if _, ok := registers[in.treg]; !ok {
			registers[in.treg] = 0
		}

		if in.cond(registers[in.treg], in.tval) {
			switch in.op {
			case "inc":
				registers[in.reg] += in.val
			case "dec":
				registers[in.reg] -= in.val
			default:
				panic(fmt.Sprintf("unexpected operation: %q", in.op))
			}
		}
	}

	// find the largest value in any register
	var max int
	for _, v := range registers {
		if v > max {
			max = v
		}
	}
	return max
}

func p2(input []instruction) int {
	var max int
	registers := make(map[string]int)
	for _, in := range input {
		if _, ok := registers[in.reg]; !ok {
			registers[in.reg] = 0
		}
		if _, ok := registers[in.treg]; !ok {
			registers[in.treg] = 0
		}

		if in.cond(registers[in.treg], in.tval) {
			switch in.op {
			case "inc":
				registers[in.reg] += in.val
			case "dec":
				registers[in.reg] -= in.val
			default:
				panic(fmt.Sprintf("unexpected operation: %q", in.op))
			}

			if v := registers[in.reg]; v > max {
				max = v
			}
		}
	}
	return max
}
