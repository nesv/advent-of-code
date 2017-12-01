package main

import (
	"bufio"
	"bytes"
	"flag"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

var (
	BufferSize int
)

func init() {
	flag.IntVar(&BufferSize, "bufsize", 64*1024, "Size of the internal buffered reader")
	flag.Parse()
}

func main() {
	n, err := Decompress(os.Stdout, bufio.NewReaderSize(os.Stdin, BufferSize))
	if err != nil && err != io.EOF {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Fprintln(os.Stderr, ">>>", n)
}

func write(w io.Writer, p []byte) (int, error) {
	if len(p) == 0 {
		return 0, nil
	}
	n, err := w.Write(bytes.Trim(p, " \t\n"))
	fmt.Fprintf(os.Stderr, "WRITE %d %q\n", n, string(p))
	return n, err
}

func Decompress(dst io.Writer, src *bufio.Reader) (int, error) {
	var size int
	for {
		// Read until the first opening parenthesis.
		p, err := src.ReadBytes('(')
		if err != nil && err == io.EOF {
			size += len(p)
			_, err := dst.Write(p)
			return size, err
		} else if err != nil {
			return size, err
		}

		// Drop the opening parenthesis.
		p = p[:len(p)-1]

		// Write out the bytes we have just read (minus the opening
		// parenthesis).
		n, err := write(dst, p)
		if err != nil {
			return size, err
		}
		size += n

		// Read until we find a matching closing parenthesis.
		rep, err := src.ReadBytes(')')
		if err != nil && err == io.EOF {
			// We have read until EOF, so just dump the read bytes.
			n, err := write(dst, rep)
			size += n
			return size, err
		} else if err != nil {
			return size, err
		}

		// Parse the repetition marker, then read nchars bytes, and
		// write those bytes ntimes to dst.
		nchars, ntimes, err := ParseRepetition(string(rep))
		if err != nil {
			return size, err
		}
		repStr := make([]byte, nchars)
		n, err = src.Read(repStr)
		if err != nil && err == io.EOF {
			n, err := write(dst, repStr[:n])
			size += n
			return size, err
		} else if err != nil {
			return size, err
		}
		fmt.Fprintf(os.Stderr, "REP %d %d %q\n", nchars, ntimes, string(repStr))
		for i := 0; i < ntimes; i++ {
			n, err := write(dst, repStr)
			if err != nil {
				return size, err
			}
			size += n
		}
	}
}

func ParseRepetition(s string) (numChars, times int, err error) {
	// Trim the parentheses.
	s = strings.Trim(s, "()")
	sp := strings.Split(s, "x")
	numChars, err = strconv.Atoi(sp[0])
	if err != nil {
		return 0, 0, err
	}
	times, err = strconv.Atoi(sp[1])
	if err != nil {
		return 0, 0, err
	}

	return numChars, times, nil
}
