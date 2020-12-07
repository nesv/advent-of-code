package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

func main() {
	rules, err := rulesFrom(os.Stdin)
	if err != nil {
		log.Fatalln(err)
	}
	//fmt.Fprintln(os.Stderr, rules)
	bags := rules.contains("shiny gold")
	//fmt.Fprintln(os.Stderr, bags)
	fmt.Println("Part 1:", len(bags))

	fmt.Println("Part 2:", rules.nbags("shiny gold"))
}

var (
	containingRegexp = regexp.MustCompile(`(\S.+?) (\S.+?) bags?`)
	containedRegexp  = regexp.MustCompile(`(no other bags|(\d+?) (\S.+?) (\S.+?) bags?)[,.]?`)
)

// rulesFrom reads the rules from r, and returns them in a map where the key
// is a colour of bag, and the values are the colour of bags that may contain
// the key.
// It is a reverse association of the raw rules.
func rulesFrom(r io.Reader) (*rules, error) {
	var (
		bb = make(map[string]map[string]int)
		m  = make(map[string]map[string]struct{})
	)

	sc := bufio.NewScanner(r)
Outer:
	for sc.Scan() {
		parts := strings.SplitN(sc.Text(), "contain", 2)
		if len(parts) != 2 {
			return nil, fmt.Errorf("malformed rule? %q", sc.Text())
		}

		// Parse the containing bag.
		var containing string
		matches := containingRegexp.FindStringSubmatch(parts[0])
		if len(matches) == 3 {
			containing = fmt.Sprintf("%s %s", matches[1], matches[2])
		} else {
			return nil, fmt.Errorf("expected two matches for %q, got %d", parts[0], len(matches))
		}
		//fmt.Fprintln(os.Stderr, containing)

		// Add an entry to the forward mapping.
		if _, ok := bb[containing]; ok {
			panic("mapping already exists: " + containing)
		}
		bb[containing] = make(map[string]int)

		// Parse the bags that it may contain.
		bags := strings.Split(parts[1], `,`)
		for _, b := range bags {
			matches := containedRegexp.FindStringSubmatch(b)
			//fmt.Fprintf(os.Stderr, "matches=%d: %#v\n", len(matches), matches)
			if len(matches) != 5 {
				return nil, fmt.Errorf("too few matches for %q", b)
			}

			if matches[1] == "no other bags" {
				continue Outer
			}

			n, err := strconv.Atoi(matches[2])
			if err != nil {
				return nil, fmt.Errorf("atoi %q from %q: %w", matches[2], b, err)
			}
			bag := fmt.Sprintf("%s %s", matches[3], matches[4])
			//fmt.Fprintln(os.Stderr, "-", bag)

			// Add an entry for the forward mapping.
			bb[containing][bag] = n

			// Add the entry to the reverse mapping.
			containedBy, ok := m[bag]
			if !ok {
				containedBy = make(map[string]struct{})
			}
			containedBy[containing] = struct{}{}
			m[bag] = containedBy

		}
	}
	if err := sc.Err(); err != nil {
		return nil, err
	}

	return &rules{
		bags: bb,
		m:    m,
	}, nil
}

type rules struct {
	// Mapping of bag, to how many of each bag it can hold.
	// color -> color -> num
	bags map[string]map[string]int

	// Reverse mapping of bag-(contained-by)->bag.
	m map[string]map[string]struct{}
}

// contains returns the bags that may eventually contain a bag of
// the given color.
func (r rules) contains(bag string) []string {
	m := make(map[string]struct{})

	containedBy, ok := r.m[bag]
	if !ok {
		return nil
	}
	//fmt.Fprintln(os.Stderr, bag)
	for b, _ := range containedBy {
		m[b] = struct{}{}
		//fmt.Fprintln(os.Stderr, "-", b)
		for _, b := range r.contains(b) {
			m[b] = struct{}{}
		}
	}

	var bags []string
	for b, _ := range m {
		bags = append(bags, b)
	}
	sort.Strings(bags)
	return bags
}

// nbags returns the number of bags to purchase given a particular color.
// nbags will return 0 if it cannot find a bag.
func (r rules) nbags(bag string) int {
	m, ok := r.bags[bag]
	if !ok {
		return 0
	}

	var n int
	for b, count := range m {
		n += count + (count * r.nbags(b))
		//fmt.Fprintf(os.Stderr, "%s + %d*%s = %d\n", bag, count, b, n)
	}
	return n
}

// rules returns a list of bag colours for which there are rules.
func (r rules) rules() []string {
	var colors []string
	for c, _ := range r.bags {
		colors = append(colors, c)
	}
	sort.Strings(colors)
	return colors
}

func (r rules) String() string {
	var b strings.Builder
	for k, vv := range r.m {
		fmt.Fprintf(&b, "%s: ", k)
		for l, _ := range vv {
			fmt.Fprintf(&b, "%q ", l)
		}
		fmt.Fprintln(&b, "")
	}
	return b.String()
}
