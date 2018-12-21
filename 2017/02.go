package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"math/bits"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	test()

	f, err := os.Open("input.2")
	if err != nil {
		log.Fatalln(err)
	}
	defer f.Close()

	input := toIntSlice(f)

	fmt.Println(p1(input))
	fmt.Println(p2(input))
}

func test() {
	sample := `5 1 9 5
7 5 3
2 4 6 8`
	want := 18
	if got := p1(toIntSlice(strings.NewReader(sample))); got != want {
		log.Fatalf("error: wanted %d, got %d", want, got)
	}

	sample = `5 9 2 8
9 4 7 3
3 8 6 5`
	want = 9
	if got := p2(toIntSlice(strings.NewReader(sample))); got != want {
		log.Fatalf("error: wanted %d, got %d", want, got)
	}
}

func p1(input [][]int) int {
	var sum int
	for i := range input {
		row := input[i]
		sort.Ints(row)
		sum += row[len(row)-1] - row[0]
	}
	return sum
}

func toIntSlice(r io.Reader) [][]int {
	var g [][]int

	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		f := strings.Fields(scanner.Text())
		row := make([]int, len(f))
		for i, v := range f {
			if n, err := strconv.Atoi(v); err != nil {
				log.Fatalln(err)
			} else {
				row[i] = n
			}
		}
		g = append(g, row)
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln(err)
	}

	return g
}

func p2(input [][]int) int {
	var sum int
	for _, row := range input {
		row := row
		sort.Ints(row)
		for i := range row {
			for j := i + 1; j < len(row); j++ {
				if q, r := divmod(row[j], row[i]); r == 0 {
					sum += q
				}
			}
		}
	}
	return sum
}

func divmod(a, b int) (q, r int) {
	if b == 0 {
		panic("divide by zero")
	}

	var na, nb bool
	if a < 0 {
		na = true
		a = bneg(a)
	}
	if b < 0 {
		nb = true
		b = bneg(b)
	}

	q, r = 0, a

	n := uint(bits.LeadingZeros(uint(b)) - bits.LeadingZeros(uint(a)))
	d := b << n

	for r >= b {
		for d > r {
			d >>= 1
			q <<= 1
			n--
		}
		r = bsub(r, d)
		q |= 1
	}
	q <<= n

	if na {
		r = bneg(r)
		if !nb {
			q = bneg(q)
		}
	} else if nb {
		q = bneg(q)
	}

	return q, r
}

func bneg(a int) int {
	return badd(^a, 1)
}

func badd(a, b int) int {
	for b != 0 {
		a, b = a^b, (a&b)<<1
	}
	return a
}

func bsub(a, b int) int {
	return badd(a, bneg(b))
}
