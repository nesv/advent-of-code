package main

import (
	"fmt"
	"log"
	"math"
)

const input = 265149

const sample = `17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23  24  25`

func test() {
	for in, want := range map[int]int{
		1:    0,
		12:   3,
		23:   2,
		1024: 31,
	} {
		if got := p1(in); got != want {
			log.Fatalf("input=%d: want=%d, got=%d", in, want, got)
		}
	}
}

func main() {
	test()
	fmt.Println(p1(input))
	fmt.Println(p2(input))
}

func p1(n int) int {
	x, y := coords(n)
	return x + y
}

func coords(n int) (p, q int) {
	var steps, turns int
	var x, y float64
	for steps < n-1 {
		for i := 0; i < (turns/2)+1; i++ {
			if steps == n-1 {
				break
			}
			steps++
			switch turns % 4 {
			case 0:
				x++
			case 1:
				y++
			case 2:
				x--
			default:
				y--
			}
		}
		turns++
	}

	return int(math.Abs(x)), int(math.Abs(y))
}

func sumAround(chart map[string]int, n int) int {
	var sum int
	x, y := coords(n)
	for i := -1; i < 2; i++ {
		for j := -1; j < 2; j++ {
			if i == 0 && y == 0 {
				continue
			}

			k := fmt.Sprintf("%d,%d", x+i, y+j)
			if v, ok := chart[k]; ok {
				sum += v
			}
		}
	}
	return sum
}

func p2(n int) int {
	var num int

	chart := map[string]int{"0,0": 1}
	for i := 1; ; i++ {
		x, y := coords(i - 1)
		k := fmt.Sprintf("%d,%d", x, y)
		if v, ok := chart[k]; ok && v > n {
			return num
		}

		x, y = coords(i)
		k = fmt.Sprintf("%d,%d", x, y)
		num = sumAround(chart, i)
		chart[k] = num

		log.Printf("adding to chart: (%s)=%d", k, num)
	}
}
