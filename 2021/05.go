package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

//go:embed input/05
var input string

func main() {
	segments, err := parseLineSegments()
	if err != nil {
		fmt.Fprintln(os.Stderr, "parse line segments:", err)
		os.Exit(1)
	}

	// Part 1: Find the number of points where 2 or more lines intersect,
	// only considering lines that are vertical and horizontal.
	fmt.Println(part1(segments))

	// Part 2: Find the number of points where 2 or more line segments
	// intersection.
	// Consider all lines, this time.
	fmt.Println(part2(segments))
}

func parseLineSegments() ([]lineSegment, error) {
	var (
		segments []lineSegment
		scanner  = bufio.NewScanner(strings.NewReader(input))
	)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		if len(fields) != 3 {
			return nil, fmt.Errorf("expected three fields, got %d: %v", len(fields), fields)
		}

		start, err := parsePoint(fields[0])
		if err != nil {
			return nil, fmt.Errorf("parse starting point: %w", err)
		}

		end, err := parsePoint(fields[2])
		if err != nil {
			return nil, fmt.Errorf("parse ending point: %w", err)
		}

		segments = append(segments, lineSegment{start: start, end: end})
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return segments, nil
}

func parsePoint(s string) (point, error) {
	i := strings.Index(s, ",")
	if i == -1 {
		return point{}, fmt.Errorf("no separator in %q", s)
	}

	x, err := strconv.Atoi(s[:i])
	if err != nil {
		return point{}, fmt.Errorf("parse x coordinate: %w", err)
	}

	y, err := strconv.Atoi(s[i+1:])
	if err != nil {
		return point{}, fmt.Errorf("parse y coordinate: %w", err)
	}

	return point{x: x, y: y}, nil
}

type lineSegment struct {
	start, end point
}

func (ls lineSegment) points() []point {
	var (
		points []point

		dx = cmp(ls.start.x, ls.end.x)
		dy = cmp(ls.start.y, ls.end.y)
	)

	for x, y := ls.start.x, ls.start.y; x != ls.end.x || y != ls.end.y; {
		points = append(points, point{x: x, y: y})
		x += dx
		y += dy
	}

	points = append(points, ls.end)
	return points
}

func cmp(a, b int) int {
	if b > a {
		return 1
	} else if b < a {
		return -1
	}
	return 0
}

func (ls lineSegment) String() string {
	return fmt.Sprintf("%s -> %s", ls.start, ls.end)
}

func (ls lineSegment) isHorizontal() bool {
	return ls.start.y == ls.end.y
}

func (ls lineSegment) isVertical() bool {
	return ls.start.x == ls.end.x
}

type point struct {
	x, y int
}

func (p point) String() string {
	return fmt.Sprintf("%d,%d", p.x, p.y)
}

// plotSegments plots all of the given line segments and returns the x->y->n
// mapping, where "n" is the number of line segments that overlap on that
// point.
func plotSegments(segments []lineSegment) grid {
	g := make(map[int]map[int]int)
	for _, s := range segments {
		for _, p := range s.points() {
			if _, ok := g[p.x]; !ok {
				g[p.x] = make(map[int]int)
			}
			g[p.x][p.y]++
		}
	}
	return g
}

type grid map[int]map[int]int

func (g grid) score() int {
	var score int
	for _, n := range g {
		for _, c := range n {
			if c >= 2 {
				score++
			}
		}
	}
	return score
}

// part1 only considers vertical and horizontal line segments.
func part1(segments []lineSegment) int {
	var segs []lineSegment
	for _, s := range segments {
		if !s.isVertical() && !s.isHorizontal() {
			continue
		}
		segs = append(segs, s)
	}

	g := plotSegments(segs)
	return g.score()
}

func part2(segments []lineSegment) int {
	g := plotSegments(segments)
	return g.score()
}
