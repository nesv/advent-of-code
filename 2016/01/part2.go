package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

func main() {
	p, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	direction := "n"
	x, y := 0, 0
	visited := map[int]map[int]bool{0: {0: true}}
	fmt.Println(x, y)
	for _, dir := range strings.Split(string(p), ",") {
		dir = strings.Trim(dir, " \n")
		switch dir[0] {
		case 'R':
			switch direction {
			case "n":
				direction = "e"
			case "e":
				direction = "s"
			case "w":
				direction = "n"
			case "s":
				direction = "w"
			}
		case 'L':
			switch direction {
			case "n":
				direction = "w"
			case "e":
				direction = "n"
			case "w":
				direction = "s"
			case "s":
				direction = "e"
			}
		default:
			fmt.Printf("weird input: %v\n", dir[0])
			os.Exit(1)
		}

		blocks, err := strconv.Atoi(dir[1:])
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		switch direction {
		case "n":
			for i := y + 1; i < y+blocks; i++ {
				fmt.Println(x, i)
				if _, ok := visited[x]; !ok {
					visited[x] = make(map[int]bool)
				}
				if _, ok := visited[x][i]; ok {
					fmt.Println("^^^")
					printAbs(x, i)
					return
				}
				visited[x][i] = true
			}
			y += blocks
		case "s":
			for i := y - 1; i > y-blocks; i-- {
				fmt.Println(x, i)
				if _, ok := visited[x]; !ok {
					visited[x] = make(map[int]bool)
				}
				if _, ok := visited[x][i]; ok {
					fmt.Println("^^^")
					printAbs(x, i)
					return
				}
				visited[x][i] = true
			}
			y -= blocks
		case "e":
			for i := x + 1; i < x+blocks; i++ {
				fmt.Println(i, y)
				if _, ok := visited[i]; !ok {
					visited[i] = make(map[int]bool)
				}
				if _, ok := visited[i][y]; ok {
					fmt.Println("^^^")
					printAbs(i, y)
					return
				}
				visited[i][y] = true
			}
			x += blocks
		case "w":
			for i := x - 1; i > x-blocks; i-- {
				fmt.Println(i, y)
				if _, ok := visited[i]; !ok {
					visited[i] = make(map[int]bool)
				}
				if _, ok := visited[i][y]; ok {
					fmt.Println("^^^")
					printAbs(i, y)
					return
				}
				visited[i][y] = true
			}
			x -= blocks
		}
	}
}

func printAbs(a, b int) {
	if a < 0 {
		a *= -1
	}
	if b < 0 {
		b *= -1
	}
	fmt.Println(a + b)
}
