package main

import (
	"crypto/md5"
	"flag"
	"fmt"
	"strings"
)

var prefix = []byte{'0', '0', '0', '0', '0'}

func main() {
	flag.Parse()
	if flag.NArg() < 1 {
		fmt.Println("no input provided")
		return
	}

	input := flag.Arg(0)
	var found int
	var passwd []byte
	for i := 0; ; i++ {
		if found == 8 {
			break
		}
		in := fmt.Sprintf("%s%d", input, i)
		sum := md5.Sum([]byte(in))
		hex := fmt.Sprintf("%x", sum)
		fmt.Printf("md5(%s) => %s\n", in, hex)
		if strings.HasPrefix(hex, "00000") {
			passwd = append(passwd, hex[5])
			found++
		}
	}
	fmt.Println(string(passwd))
}
