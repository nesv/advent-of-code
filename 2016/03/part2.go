package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var valid int
	var tri = [][]int{
		make([]int, 3, 3),
		make([]int, 3, 3),
		make([]int, 3, 3),
	}
	scanner := bufio.NewScanner(os.Stdin)
Outer:
	for {
		for i := 0; i < 3; i++ {
			if scanner.Scan() {
				var a, b, c int
				if n, err := fmt.Sscanf(strings.TrimSpace(scanner.Text()), "%d %d %d\n",
					&a,
					&b,
					&c,
				); err != nil {
					fmt.Println(err)
					os.Exit(1)
				} else if n != 3 {
					fmt.Println("wanted 3 args, got", n)
					os.Exit(1)
				}
				tri[0][i], tri[1][i], tri[2][i] = a, b, c
			} else {
				break Outer
			}
		}

		for i := 0; i < 3; i++ {
			x, y, z := tri[i][0], tri[i][1], tri[i][2]
			fmt.Printf("%d %d %d\n", x, y, z)
			if x+y > z && x+z > y && y+z > x {
				valid++
			}
		}
	}
	fmt.Println(valid)
}
