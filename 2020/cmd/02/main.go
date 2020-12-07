package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	var entries []entry
	sc := bufio.NewScanner(os.Stdin)
	for sc.Scan() {
		var (
			min, max       int
			char, password string
		)
		if _, err := fmt.Sscanf(sc.Text(), "%d-%d %s: %s", &min, &max, &char, &password); err != nil {
			log.Fatalf("parse %q: %v", sc.Text(), err)
		}
		entries = append(entries, entry{
			min:      min,
			max:      max,
			char:     char,
			password: password,
		})
	}
	if err := sc.Err(); err != nil {
		log.Fatalln("read input:", err)
	}

	var n, m int
	for _, e := range entries {
		if e.valid() {
			n++
		}
		if e.valid2() {
			m++
		}
	}

	fmt.Printf("%d\n%d\n", n, m)
}

type entry struct {
	min, max       int
	char, password string
}

func (e entry) valid() bool {
	n := strings.Count(e.password, e.char)
	return n >= e.min && n <= e.max
}

func (e entry) valid2() bool {
	a := string(e.password[e.min-1]) == e.char
	b := string(e.password[e.max-1]) == e.char
	return (a || b) && !(a && b)
}
