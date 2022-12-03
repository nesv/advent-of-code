package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"log"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	var rucksacks []string
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		rucksacks = append(rucksacks, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan input:", err)
	}

	priority := make(map[rune]int, 52)
	for i := 1; i < 27; i++ {
		priority[rune(i+96)] = i
	}
	for i := 27; i < 53; i++ {
		priority[rune(i+38)] = i
	}

	// Part 1: Find the item type that appears in both compartments of each rucksack.
	// What is the sum of the priorities of those item types?
	//
	// To help prioritize item rearrangement, every item type can be converted to a priority:
	//   - Lowercase item types a through z have priorities 1 through 26.
	//   - Uppercase item types A through Z have priorities 27 through 52.
	var psum int
	for _, r := range rucksacks {
		n := len(r) / 2
	rucksack:
		for _, c := range r[:n] {
			for _, d := range r[n:] {
				if c == d {
					// log.Printf("rucksack(%d): found %c\n", i, c)
					psum += priority[c]
					break rucksack
				}
			}
		}
	}
	fmt.Println(psum)

	// Part 2: Find the item type that corresponds to the badges of each three-Elf group.
	// What is the sum of the priorities of those item types?
	//
	// The only way to tell which item type is the right one is by finding the
	// one item type that is common between all three Elves in each group.
	psum = 0
	for i := 0; i < len(rucksacks); i += 3 {
	r1:
		for _, c := range rucksacks[i] {
			for _, d := range rucksacks[i+1] {
				// If we do not get a match on the current rune,
				// skip ahead to the next rune.
				if c != d {
					continue
				}

				// We found a batch between the two rucksacks!
				// Check to see if the third rucksack has the same item.
				for _, e := range rucksacks[i+2] {
					if c == e {
						psum += priority[c]
						break r1
					}
				}
			}
		}
	}
	fmt.Println(psum)
}
