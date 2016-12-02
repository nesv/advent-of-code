package main

import (
	"bufio"
	"fmt"
	"os"
)

var keypad = [][]rune{
	{0, 0, '1', 0, 0},
	{0, '2', '3', '4', 0},
	{'5', '6', '7', '8', '9'},
	{0, 'A', 'B', 'C', 0},
	{0, 0, 'D', 0, 0},
}

var move = map[rune]func(x, y int) (nx, ny int){
	'L': func(x, y int) (int, int) {
		if y-1 < 0 || keypad[x][y-1] == 0 {
			return x, y
		}
		return x, y - 1
	},
	'R': func(x, y int) (int, int) {
		if y+1 >= len(keypad) || keypad[x][y+1] == 0 {
			return x, y
		}
		return x, y + 1
	},
	'U': func(x, y int) (int, int) {
		if x-1 < 0 || keypad[x-1][y] == 0 {
			return x, y
		}
		return x - 1, y
	},
	'D': func(x, y int) (int, int) {
		if x+1 >= len(keypad[x]) || keypad[x+1][y] == 0 {
			return x, y
		}
		return x + 1, y
	},
}

func main() {
	x, y := 2, 0 // Start at "5"

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

		fmt.Printf("%s ", string(keypad[x][y]))
		//fmt.Println(string(keypad[x][y]))
	}
	fmt.Println()
}
