// Advent of Code 2020: Day 1.
package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	// Read input from STDIN.
	var entries []int
	sc := bufio.NewScanner(os.Stdin)
	for sc.Scan() {
		n, err := strconv.Atoi(sc.Text())
		if err != nil {
			log.Fatal(err)
		}
		entries = append(entries, n)
	}

	a, b := findTwo(2020, entries)
	fmt.Printf("%d * %d = %d\n", a, b, a*b)

	a, b, c := findThree(2020, entries)
	fmt.Printf("%d * %d * %d = %d\n", a, b, c, a*b*c)
}

func findTwo(target int, nums []int) (a, b int) {
	for i := 0; i < len(nums)-2; i++ {
		for j := i + 1; j < len(nums)-1; j++ {
			if nums[i]+nums[j] == target {
				return nums[i], nums[j]
			}
		}
	}
	return 0, 0
}

func findThree(target int, nums []int) (a, b, c int) {
	for i := 0; i < len(nums)-3; i++ {
		for j := i + 1; j < len(nums)-2; j++ {
			for k := i + 2; k < len(nums)-1; k++ {
				if nums[i]+nums[j]+nums[k] == target {
					return nums[i], nums[j], nums[k]
				}
			}
		}
	}
	return 0, 0, 0
}
