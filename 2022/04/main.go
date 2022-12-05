package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"log"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	var (
		ranges  []inclusiveRange
		r       = strings.NewReader(input)
		scanner = bufio.NewScanner(r)
	)
	for scanner.Scan() {
		for _, field := range strings.Split(scanner.Text(), ",") {
			i := strings.Index(field, "-")
			if i == -1 {
				log.Fatalf("expected '-' in %q\n", scanner.Text())
			}

			start, err := strconv.Atoi(field[:i])
			if err != nil {
				log.Fatalln("atoi:", err)
			}

			end, err := strconv.Atoi(field[i+1:])
			if err != nil {
				log.Fatalln("atoi:", err)
			}

			ranges = append(ranges, inclusiveRange{
				start: start,
				end:   end,
			})
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan input:", err)
	}

	// Part 1: In how many assignment pairs does one range fully contain the other?
	var contained int
	for i := 0; i < len(ranges); i += 2 {
		var (
			a = ranges[i]
			b = ranges[i+1]
		)
		if a.contains(b) || b.contains(a) {
			contained++
		}
	}
	fmt.Println(contained)

	// Part 2: In how many assignment pairs do the ranges overlap?
	var overlapped int
	for i := 0; i < len(ranges); i += 2 {
		var (
			a = ranges[i]
			b = ranges[i+1]
		)
		if a.overlaps(b) || b.overlaps(a) {
			overlapped++
		}
	}
	fmt.Println(overlapped)
}

type inclusiveRange struct {
	start, end int
}

func (r inclusiveRange) contains(other inclusiveRange) bool {
	return r.start <= other.start && r.end >= other.end
}

func (r inclusiveRange) overlaps(other inclusiveRange) bool {
	return (r.start >= other.start && r.start <= other.end) || (r.end >= other.start && r.end <= other.end)
}
