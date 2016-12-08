package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	screen := new([6][50]bool)
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		switch fields[0] {
		case "rect":
			x, y := parseRect(fields[1])
			drawRect(screen, x, y)
		case "rotate":
			switch fields[1] {
			case "column":
				col, err := strconv.Atoi(strings.Split(fields[2], "=")[1])
				if err != nil {
					panic(err)
				}
				by, err := strconv.Atoi(fields[4])
				if err != nil {
					panic(err)
				}

				shiftCol(screen, col, by)

			case "row":
				var row, by int
				var err error
				if row, err = strconv.Atoi(strings.Split(fields[2], "=")[1]); err != nil {
					panic(err)
				}
				if by, err = strconv.Atoi(fields[4]); err != nil {
					panic(err)
				}

				shiftRow(screen, row, by)
			}
		}
		drawScreen(screen)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	fmt.Println(">>>", countPixels(screen), "pixels lit")
}

func drawScreen(scr *[6][50]bool) {
	time.Sleep(10 * time.Millisecond)
	fmt.Print("\033c\u250C")
	for i := 0; i < 50; i++ {
		fmt.Print("\u2500")
	}
	fmt.Println("\u2510")
	for y := 0; y < len(scr); y++ {
		fmt.Print("\u2502")
		for x := 0; x < len(scr[y]); x++ {
			if scr[y][x] {
				//fmt.Print("\u2573")
				//fmt.Print("#")
				//fmt.Print("\u203B")
				fmt.Print("\u220e")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println("\u2502")
	}
	fmt.Print("\u2514")
	for i := 0; i < 50; i++ {
		fmt.Print("\u2500")
	}
	fmt.Println("\u2518")
}

func shiftRow(screen *[6][50]bool, n, by int) {
	var row [50]bool
	for i := 0; i < len(row); i++ {
		if screen[n][i] {
			if i+by >= len(row) {
				row[i+by-len(row)] = true // wrapping
			} else {
				row[i+by] = true
			}
		}
	}
	screen[n] = row
}

func shiftCol(screen *[6][50]bool, n, by int) {
	var col [6]bool
	for i := 0; i < len(screen); i++ {
		if screen[i][n] {
			if i+by >= len(col) {
				col[i+by-len(col)] = true // wrapping
			} else {
				col[i+by] = true
			}
		}
	}
	for i := 0; i < len(col); i++ {
		screen[i][n] = col[i]
	}
}

func countPixels(screen *[6][50]bool) int {
	var n int
	for y := 0; y < len(screen); y++ {
		for x := 0; x < len(screen[y]); x++ {
			if screen[y][x] {
				n++
			}
		}
	}
	return n
}

func drawRect(screen *[6][50]bool, x, y int) {
	for r := 0; r < y; r++ {
		for c := 0; c < x; c++ {
			screen[r][c] = true
		}
	}
}

func parseRect(dims string) (x, y int) {
	d := strings.Split(dims, "x")
	var err error
	x, err = strconv.Atoi(d[0])
	if err != nil {
		panic(err)
	}
	y, err = strconv.Atoi(d[1])
	if err != nil {
		panic(err)
	}
	return x, y
}
