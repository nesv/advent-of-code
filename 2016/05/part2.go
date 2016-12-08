package main

import (
	"crypto/md5"
	"flag"
	"fmt"
	"strings"
)

func main() {
	flag.Parse()
	if flag.NArg() < 1 {
		fmt.Println("no input provided")
		return
	}

	input := flag.Arg(0)
	var found int
	var passwd = []byte("________")
	fmt.Print(string(passwd))
	for i := 0; ; i++ {
		if found == 8 {
			break
		}
		in := fmt.Sprintf("%s%d", input, i)
		sum := md5.Sum([]byte(in))
		hex := fmt.Sprintf("%x", sum)
		fmt.Printf("\r%s %s", string(passwd), hex)
		if strings.HasPrefix(hex, "00000") {
			//fmt.Printf("md5(%s)=>%s\n", in, hex)
			xb := []byte(hex)
			idx := xb[5] - 48 // Offset for ASCII chars
			if idx < 8 && passwd[idx] == '_' {
				passwd[idx] = xb[6]
				found++
			}
		}
	}
	fmt.Printf("\r%s\n", string(passwd))
}
