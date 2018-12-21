package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
	"strings"
)

func main() {
	in, err := os.Open("input.4")
	if err != nil {
		panic(err)
	}
	defer in.Close()

	fmt.Println(p1(in))
	in.Seek(0, 0)
	fmt.Println(p2(in))
}

func p1(r io.Reader) int {
	var valid int

	scanner := bufio.NewScanner(r)
SCAN:
	for scanner.Scan() {
		m := make(map[string]int)
		f := strings.Fields(scanner.Text())
		for _, v := range f {
			m[v]++
		}
		for _, v := range m {
			if v > 1 {
				continue SCAN
			}
		}
		valid++
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return valid
}

func p2(r io.Reader) int {
	var valid int

	scanner := bufio.NewScanner(r)
SCAN:
	for scanner.Scan() {
		m := make(map[string]int)
		f := strings.Fields(scanner.Text())

		// Sort the characters in each element of f.
		for i := range f {
			p := []byte(f[i])
			sort.Slice(p, func(n, m int) bool {
				return p[n] < p[m]
			})
			f[i] = string(p)
		}

		for _, v := range f {
			m[v]++
		}
		for _, v := range m {
			if v > 1 {
				continue SCAN
			}
		}
		valid++
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return valid
}
