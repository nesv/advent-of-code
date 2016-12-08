package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var count int
	scanner := bufio.NewScanner(os.Stdin)
ScanLoop:
	for scanner.Scan() {
		var sections, hypernets []string
		var buffer []byte
		v := scanner.Text()
		for i := 0; i < len(v); i++ {
			switch v[i] {
			case '[':
				sections = append(sections, string(buffer))
				buffer = nil
				continue
			case ']':
				hypernets = append(hypernets, string(buffer))
				buffer = nil
				continue
			}
			buffer = append(buffer, v[i])
			if i == len(v)-1 {
				sections = append(sections, string(buffer))
			}
		}

		for _, v := range hypernets {
			//fmt.Println("hypernet:", v, "?", hasABBA(v))
			if hasABBA(v) {
				continue ScanLoop
			}
		}
		for _, v := range sections {
			//fmt.Println("sections:", v, "?", hasABBA(v))
			if hasABBA(v) {
				count++
				continue ScanLoop
			}
		}
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("error:", err)
		os.Exit(1)
	}
	fmt.Println(count)
}

func hasABBA(s string) bool {
	for i := 0; i < len(s)-3; i++ {
		if s[i] != s[i+1] && s[i] == s[i+3] && s[i+1] == s[i+2] {
			return true
		}
	}
	return false
}
