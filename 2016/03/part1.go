package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var valid int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		if err := scanner.Err(); err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

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

		fmt.Printf("%d + %d > %d => %v\n", a, b, c, (a+b > c))
		if a+b > c && b+c > a && a+c > b {
			valid++
		}
	}
	fmt.Println(valid)
}
