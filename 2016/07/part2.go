package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var count int
	scanner := bufio.NewScanner(os.Stdin)
ScanLoop:
	for scanner.Scan() {
		var supernets, hypernets []string
		var buffer []byte
		v := scanner.Text()
		for i := 0; i < len(v); i++ {
			switch v[i] {
			case '[':
				supernets = append(supernets, string(buffer))
				buffer = nil
				continue
			case ']':
				hypernets = append(hypernets, string(buffer))
				buffer = nil
				continue
			}
			buffer = append(buffer, v[i])
			if i == len(v)-1 {
				supernets = append(supernets, string(buffer))
			}
		}

		var aba []string
		for _, v := range supernets {
			aba = append(aba, findAllABA(v)...)
		}
		for _, v := range hypernets {
			if hasBAB(v, aba) {
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

func findAllABA(s string) []string {
	var abas []string
	for i := 0; i < len(s)-2; i++ {
		if s[i] == s[i+2] && s[i] != s[i+1] {
			abas = append(abas, s[i:i+2])
		}
	}
	return abas
}

func hasBAB(hypernet string, aba []string) bool {
	for _, s := range aba {
		if strings.Contains(hypernet, aba2BAB(s)) {
			return true
		}
	}
	return false
}

func aba2BAB(aba string) string {
	return string([]byte{aba[1], aba[0], aba[1]})
}
