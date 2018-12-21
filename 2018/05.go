package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strings"
)

func main() {
	p, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		log.Fatalln("Failed to read from STDIN:", err)
	}

	input := strings.TrimSpace(string(p))

	fmt.Printf("Final polymer sequence length: %d\n", len(part1(input)))
	fmt.Printf("Shortest-possible polymer sequence length: %d\n", len(part2(input)))
}

func part1(sequence string) string {
	seq := sequence
	for {
		v := reduceSequence(seq)
		if seq == v {
			break
		}
		seq = v
	}
	return seq
}

func reduceSequence(seq string) string {
	// only destroy one sequence at a time
	for i := 0; i < len(seq); i++ {
		if i+1 == len(seq) {
			break
		}

		c, n := seq[i], seq[i+1]
		if (c > n && c-n == 32) || n-c == 32 {
			return seq[:i] + seq[i+2:]
		}
	}

	return seq
}

// Loop through all 26 characters of the English alphabet.
// Remove all upper- and lower-case instances of that letter, and see which
// unit would result in the shorted sequence, once removed.
func part2(seq string) string {
	m := make(map[byte]string)
	for i := int('A'); i < int('Z')+1; i++ {
		v := strings.Map(func(r rune) rune {
			if r == rune(i) || r == rune(i+32) {
				return -1
			}
			return r
		}, seq)

		m[byte(i)] = part1(v)
	}

	shortest := seq
	for _, s := range m {
		if len(s) < len(shortest) {
			shortest = s
		}
	}
	return shortest
}
