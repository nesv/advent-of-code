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
	for _, dir := range strings.Split(string(p), ",") {
		dir = strings.Trim(dir, " ")

		steps, err := strconv.Atoi(strings.Trim(dir[1:], "\n"))
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

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

		switch direction {
		case "n":
			y += steps
		case "e":
			x += steps
		case "w":
			x -= steps
		case "s":
			y -= steps
		}
	}
	if x < 0 {
		x *= -1
	}
	if y < 0 {
		y *= -1
	}
	fmt.Println(x + y)
}
