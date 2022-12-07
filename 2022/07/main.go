package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"log"
	"path/filepath"
	"sort"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	var (
		r       = strings.NewReader(input)
		scanner = bufio.NewScanner(r)

		pwd  string
		tree = make(map[string]int)
	)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		switch fields[0] {
		case "$":
			// Parse a command.
			switch cmd := fields[1]; cmd {
			case "cd":
				// Change directory.
				switch dir := fields[2]; dir {
				case "..":
					// Go up one directory.
					pwd = filepath.Dir(pwd)
				case "/":
					// Go to the root directory.
					pwd = "/"
				default:
					pwd = filepath.Join(pwd, dir)
				}

			case "ls":
				// List files in the current directory.
			}

		case "dir":
			// Parse a directory from a directory listing.
			path := filepath.Join(pwd, fields[1])
			tree[path] = 0

		default:
			// Parse a file from a directory listing.
			var (
				size int
				name string
			)
			if _, err := fmt.Sscanf(scanner.Text(), "%d %s\n", &size, &name); err != nil {
				log.Fatalf("parse file entry %q: %v\n", scanner.Text(), err)
			}

			path := filepath.Join(pwd, name)
			tree[path] = size
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatalln("scan input:", err)
	}

	// Part 1: Find all of the directories with a total size of at most 100000.
	// What is the sum of the total sizes of those directories?
	dirSizes := make(map[string]int)
	for path, size := range tree {
		log.Printf("%s => %d\n", path, size)
		for dir := filepath.Dir(path); ; dir = filepath.Dir(dir) {
			dsize, _ := dirSizes[dir]
			dsize += size
			dirSizes[dir] = dsize
			if dir == "/" {
				break
			}
		}
	}

	var sum int
	for _, size := range dirSizes {
		if size <= 100000 {
			sum += size
		}
	}
	fmt.Println(sum)

	// Part 2: Find the size of the smallest directory we could delete,
	// that would give us at least 30000000 bytes of space.
	const (
		maxDiskSpace = 70000000
		spaceNeeded  = 30000000
	)
	used := dirSizes["/"]
	log.Println("space used:", used)

	available := maxDiskSpace - used
	log.Println("space available:", available)

	// Collect the names of all directories.
	var dirs []string
	for dir := range dirSizes {
		dirs = append(dirs, dir)
	}

	// Sort directories by size.
	sort.Slice(dirs, func(i, j int) bool {
		var (
			a = dirSizes[dirs[i]]
			b = dirSizes[dirs[j]]
		)
		return a < b
	})

	// Find the first directory that, if deleted, would give us enough free
	// disk space.
	for _, dir := range dirs {
		size := dirSizes[dir]
		if available+size >= spaceNeeded {
			fmt.Println(size)
			return
		}
	}
}
