package main

import (
	"bufio"
	"fmt"
	"os"
)

var keypad = [][]int{
	{1, 2, 3},
	{4, 5, 6},
	{7, 8, 9},
}

var move = map[rune]func(x, y int) (nx, ny int){
	'L': func(x, y int) (int, int) {
		if y-1 < 0 {
			return x, y
		}
		return x, y - 1
	},
	'R': func(x, y int) (int, int) {
		if y+1 >= len(keypad) {
			return x, y
		}
		return x, y + 1
	},
	'U': func(x, y int) (int, int) {
		if x-1 < 0 {
			return x, y
		}
		return x - 1, y
	},
	'D': func(x, y int) (int, int) {
		if x+1 >= len(keypad[x]) {
			return x, y
		}
		return x + 1, y
	},
}

func main() {
	x, y := 1, 1 // Start at "5"

	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		if err := scanner.Err(); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}

		for i, char := range scanner.Text() {
			//fmt.Printf("keypad[%d][%d](%v) ", x, y, string(char))
			f, ok := move[char]
			if !ok {
				fmt.Fprintf(os.Stderr, "error: unknown char %v at position %d\n", char, i)
				os.Exit(1)
			}
			x, y = f(x, y)
			//fmt.Printf("=> keypad[%d][%d] = %d\n", x, y, keypad[x][y])
		}

		fmt.Printf("%d ", keypad[x][y])
	}
	fmt.Println()
}
