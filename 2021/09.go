// Day 9: Smoke Basin

package main

import (
	_ "embed"
	"fmt"
	"sort"
	"unicode"
)

//go:embed input/09
var input []byte

func main() {
	var (
		grid [][]byte
		row  []byte
	)
	for _, b := range input {
		if unicode.IsSpace(rune(b)) {
			//fmt.Fprintf(os.Stderr, "%v\n", row)
			grid = append(grid, row)
			row = nil
			continue
		}
		row = append(row, b-48)
	}

	// Part 1: What is the sum of the risk levels of all low points on your
	// heightmap?
	fmt.Println(part1(grid))

	// Part 2: What do you get if you multiply together the sizes of the
	// three largest basins?
	fmt.Println(part2(grid))
}

func part1(grid [][]byte) int {
	var sum int
	for _, col := range findLowPoints(grid) {
		for _, lp := range col {
			//fmt.Fprintf(os.Stderr, "%d ", lp)
			sum += int(lp) + 1
		}
	}
	//fmt.Fprint(os.Stderr, "\n")
	return sum
}

func findLowPoints(grid [][]byte) map[int]map[int]byte {
	lps := make(map[int]map[int]byte)
	for y, row := range grid {
		for x, v := range row {
			// Check above, below, to the left, and to the right
			// of the point to see if it is the lowest point in
			// its immediate vicinity.
			if y == 0 && x == 0 {
				// Top-left corner.
				if v >= grid[y+1][x] || v >= grid[y][x+1] {
					continue
				}
			} else if y == 0 && x == len(row)-1 {
				// Top-right corner.
				if v >= grid[y+1][x] || v >= grid[y][x-1] {
					continue
				}
			} else if y == len(grid)-1 && x == 0 {
				// Bottom-left corner.
				if v >= grid[y-1][x] || v >= grid[y][x+1] {
					continue
				}
			} else if y == len(grid)-1 && x == len(row)-1 {
				// Bottom-right corner.
				if v >= grid[y-1][x] || v >= grid[y][x-1] {
					continue
				}
			} else if y == 0 {
				// Top edge, not a corner.
				if v >= grid[y+1][x] || v >= grid[y][x-1] || v >= grid[y][x+1] {
					continue
				}
			} else if y == len(grid)-1 {
				// Bottom edge, not a corner.
				if v >= grid[y-1][x] || v >= grid[y][x-1] || v >= grid[y][x+1] {
					continue
				}
			} else if x == 0 {
				// Left edge, not a corner.
				if v >= grid[y-1][x] || v >= grid[y+1][x] || v >= grid[y][x+1] {
					continue
				}
			} else if x == len(row)-1 {
				// Right edge, not a corner.
				if v >= grid[y-1][x] || v >= grid[y+1][x] || v >= grid[y][x-1] {
					continue
				}
			} else if v >= grid[y-1][x] || v >= grid[y+1][x] || v >= grid[y][x-1] || v >= grid[y][x+1] {
				// Any other point.
				continue
			}

			//fmt.Fprintf(os.Stderr, "(%d,%d) = %d\n", x, y, v)
			if _, ok := lps[x]; !ok {
				lps[x] = make(map[int]byte)
			}
			lps[x][y] = v
		}
	}
	return lps
}

func part2(grid [][]byte) int {
	bss := basinSizes(grid)
	sort.Ints(bss)
	n := len(bss)
	return bss[n-1] * bss[n-2] * bss[n-3]
}

func basinSizes(grid [][]byte) []int {
	var sizes []int
	for x, col := range findLowPoints(grid) {
		for y, _ := range col {
			n := basinSize(grid, x, y)
			//fmt.Fprintf(os.Stderr, "(%d,%d) => %d\n", x, y, n)
			sizes = append(sizes, n)
		}
	}
	return sizes
}

func basinSize(grid [][]byte, x, y int) int {
	var (
		size = 1
		v    = grid[y][x]
	)
	grid[y][x] = 9

	if v == 9 {
		return 0
	}

	if y == 0 && x == 0 {
		// Top-left corner.
		if grid[y+1][x] > v {
			size += basinSize(grid, x, y+1)
		}
		if grid[y][x+1] > v {
			size += basinSize(grid, x+1, y)
		}
	} else if y == 0 && x == len(grid[0])-1 {
		// Top-right corner.
		if grid[y+1][x] > v {
			size += basinSize(grid, x, y+1)
		}
		if grid[y][x-1] > v {
			size += basinSize(grid, x-1, y)
		}
	} else if y == len(grid)-1 && x == 0 {
		// Bottom-left corner.
		if grid[y-1][x] > v {
			size += basinSize(grid, x, y-1)
		}
		if grid[y][x+1] > v {
			size += basinSize(grid, x+1, y)
		}
	} else if y == len(grid)-1 && x == len(grid[0])-1 {
		// Bottom-right corner.
		if grid[y-1][x] > v {
			size += basinSize(grid, x, y-1)
		}
		if grid[y][x-1] > v {
			size += basinSize(grid, x-1, y)
		}
	} else if y == 0 {
		// Top edge, not a corner.
		if grid[y][x-1] > v {
			size += basinSize(grid, x-1, y)
		}
		if grid[y][x+1] > v {
			size += basinSize(grid, x+1, y)
		}
		if grid[y+1][x] > v {
			size += basinSize(grid, x, y+1)
		}
	} else if y == len(grid)-1 {
		// Bottom edge, not a corner.
		if grid[y][x-1] > v {
			size += basinSize(grid, x-1, y)
		}
		if grid[y][x+1] > v {
			size += basinSize(grid, x+1, y)
		}
		if grid[y-1][x] > v {
			size += basinSize(grid, x, y-1)
		}
	} else if x == 0 {
		// Left edge, not a corner.
		if grid[y-1][x] > v {
			size += basinSize(grid, x, y-1)
		}
		if grid[y+1][x] > v {
			size += basinSize(grid, x, y+1)
		}
		if grid[y][x+1] > v {
			size += basinSize(grid, x+1, y)
		}
	} else if x == len(grid[0])-1 {
		// Right edge, not a corner.
		if grid[y-1][x] > v {
			size += basinSize(grid, x, y-1)
		}
		if grid[y+1][x] > v {
			size += basinSize(grid, x, y+1)
		}
		if grid[y][x-1] > v {
			size += basinSize(grid, x-1, y)
		}
	} else {
		// Any other point.
		if grid[y-1][x] > v {
			size += basinSize(grid, x, y-1)
		}
		if grid[y+1][x] > v {
			size += basinSize(grid, x, y+1)
		}
		if grid[y][x-1] > v {
			size += basinSize(grid, x-1, y)
		}
		if grid[y][x+1] > v {
			size += basinSize(grid, x+1, y)
		}
	}

	return size
}
