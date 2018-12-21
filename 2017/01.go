package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
)

func main() {
	test()

	in, err := ioutil.ReadFile("input.1")
	if err != nil {
		log.Fatalln(err)
	}
	in = bytes.TrimSpace(in)
	fmt.Println(p1(in))
	fmt.Println(p2(in))
}

func test() {
	for input, want := range map[string]int{
		"1122":     3,
		"1111":     4,
		"1234":     0,
		"91212129": 9,
	} {
		got := p1([]byte(input))
		if got != want {
			log.Printf("error: %s: wanted %d, got %d", input, want, got)
		}
	}

	// part 2
	for input, want := range map[string]int{
		"1212":     6,
		"1221":     0,
		"123425":   4,
		"123123":   12,
		"12131415": 4,
	} {
		if got := p2([]byte(input)); got != want {
			log.Printf("%s: wanted %d, got %d", input, want, got)
		}
	}
}

func p1(in []byte) int {
	var sum int
	for _, p := range dedup(in) {
		sum += int(p - 48)
	}
	return sum
}

func dedup(in []byte) string {
	var p []byte
	for i := 0; i < len(in); i++ {
		if i == len(in)-1 && in[i] == in[0] {
			p = append(p, in[i])
			break
		}

		if i+1 < len(in) && in[i] == in[i+1] {
			p = append(p, in[i])
		}
	}
	return string(p)
}

func p2(p []byte) int {
	var sum int
	h := len(p) / 2
	for i, c := range p {
		var j int
		if i >= h {
			j = i - h
		} else {
			j = i + h
		}
		if c == p[j] {
			sum += int(c - 48)
		}
	}
	return sum
}
