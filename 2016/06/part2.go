package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var letFreq = make(map[int]map[rune]int)

	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		for i, c := range scanner.Text() {
			if _, ok := letFreq[i]; !ok {
				letFreq[i] = make(map[rune]int)
			}
			letFreq[i][c] += 1
		}
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("error:", err)
		os.Exit(1)
	}

	msg := make([]rune, len(letFreq))
	for pos, m := range letFreq {
		var f int
		for c, freq := range m {
			if f == 0 || freq < f {
				f = freq
				msg[pos] = c
			}
		}
	}
	fmt.Println(string(msg))
}
