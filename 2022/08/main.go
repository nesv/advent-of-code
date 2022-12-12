package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"log"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	var grid [][]int // y, x => tree height
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		var row []int
		for _, char := range scanner.Text() {
			row = append(row, int(char-48))
		}
		grid = append(grid, row)
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan input:", err)
	}

	log.Printf("rows=%d cols=%d\n", len(grid), len(grid[0]))
	for _, row := range grid {
		log.Println(row)
	}

	// Part 1: How many trees are visible from outside the grid?
	fmt.Println(countVisibleTrees(grid))

	// Part 2: Find the highest scenic score for any tree.
	fmt.Println(bestScenicScore(grid))
}

func countVisibleTrees(grid [][]int) int {
	var visible int

	// Small optimization: automatically add the outer edges of the grid,
	// since those trees are always visible from outside the grid.
	visible += len(grid) * 2    // Top and bottom edges.
	visible += len(grid[0]) * 2 // Left and right edges.
	visible -= 4                // Remove the corners, since they would have been counted twice.

	for y := 1; y < len(grid)-1; y++ {
		row := grid[y]
		for x := 1; x < len(row)-1; x++ {
			isVisible := visibleFromTop(grid, y, x) || visibleFromBottom(grid, y, x) || visibleFromLeft(grid, y, x) || visibleFromRight(grid, y, x)
			log.Printf("y=%d x=%d visible=%t\n", y, x, isVisible)
			if isVisible {
				visible++
			}
		}
	}
	return visible
}

func visibleFromTop(grid [][]int, ty, tx int) bool {
	for y := ty - 1; y >= 0; y-- {
		if grid[y][tx] >= grid[ty][tx] {
			return false
		}
	}
	return true
}

func visibleFromBottom(grid [][]int, ty, tx int) bool {
	for y := ty + 1; y < len(grid); y++ {
		if grid[y][tx] >= grid[ty][tx] {
			return false
		}
	}
	return true
}

func visibleFromLeft(grid [][]int, ty, tx int) bool {
	for x := tx - 1; x >= 0; x-- {
		if grid[ty][x] >= grid[ty][tx] {
			return false
		}
	}
	return true
}

func visibleFromRight(grid [][]int, ty, tx int) bool {
	for x := tx + 1; x < len(grid[ty]); x++ {
		if grid[ty][x] >= grid[ty][tx] {
			return false
		}
	}
	return true
}

func bestScenicScore(grid [][]int) int {
	var maxScore int

	// NOTE: Skip the edges, since the scores are multiplied tgether, any tree on
	// the edge of the map will have at least one (1) direction where the number of
	// trees it can see is zero (0).
	// ...and as we know, anything multiplied by zero, is zero.
	for y := 1; y < len(grid)-1; y++ {
		for x := 1; x < len(grid[y])-1; x++ {
			score := scenicScore(grid, y, x)
			log.Printf("y=%d x=%d score=%d\n", y, x, score)
			if score > maxScore {
				maxScore = score
			}
		}
	}

	return maxScore
}

func scenicScore(grid [][]int, y, x int) int {
	up := 0
	for yy := y - 1; yy >= 0; yy-- {
		up++
		if grid[yy][x] >= grid[y][x] {
			break
		}
	}

	down := 0
	for yy := y + 1; yy < len(grid); yy++ {
		down++
		if grid[yy][x] >= grid[y][x] {
			break
		}
	}

	left := 0
	for xx := x - 1; xx >= 0; xx-- {
		left++
		if grid[y][xx] >= grid[y][x] {
			break
		}
	}

	right := 0
	for xx := x + 1; xx < len(grid[y]); xx++ {
		right++
		if grid[y][xx] >= grid[y][x] {
			break
		}
	}

	return up * down * left * right
}
