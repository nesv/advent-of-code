package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"log"
	"os"
	"strings"

	"github.com/pkg/errors"
)

func main() {
	test()

	f, err := os.Open("input.7")
	if err != nil {
		log.Fatalln(err)
	}
	defer f.Close()

	input, err := parseInput(f)
	if err != nil {
		log.Fatalln("parse input:", err)
	}

	fmt.Println(p1(input))
	fmt.Println(p2(input))
}

func parseInput(r io.Reader) ([]proc, error) {
	var procs []proc
	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		var pid string
		var weight int

		if _, err := fmt.Sscanf(scanner.Text(), "%s (%d)", &pid, &weight); err != nil {
			return nil, errors.Wrap(err, "parse pid & weight")
		}

		fields := strings.Fields(scanner.Text())
		if len(fields) == 2 {
			// this proc has no children
			procs = append(procs, proc{
				pid:    pid,
				weight: weight,
			})
			continue
		}

		children := strings.Split(strings.Join(fields[3:], ""), ",")

		procs = append(procs, proc{
			pid:      pid,
			weight:   weight,
			children: children,
		})
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return procs, nil
}

type proc struct {
	pid      string
	weight   int
	children []string

	pproc  *proc
	cprocs []*proc
}

func (p proc) totalWeight() int {
	sum := p.weight
	for _, c := range p.cprocs {
		sum += c.totalWeight()
	}
	return sum
}

func (p proc) balanced() bool {
	if len(p.cprocs) == 0 {
		return true
	}

	w := p.cprocs[0].weight
	for _, child := range p.cprocs {
		if child.weight != w {
			return false
		}
	}

	return true
}

func (p proc) unbalancedChild() *proc {
	if len(p.cprocs) == 0 {
		return nil
	}
	if p.balanced() {
		return nil
	}

	weights := p.groupChildrenByWeight()
	minLength := len(p.cprocs)
	var ucw int
	for w, childPIDs := range weights {
		if numChildren := len(childPIDs); numChildren < minLength {
			minLength = numChildren
			ucw = w
		}
	}

	for _, child := range p.cprocs {
		if child.pid == weights[ucw][0].pid {
			return child
		}
	}

	return nil
}

func (p proc) groupChildrenByWeight() map[int][]*proc {
	m := make(map[int][]*proc)
	for _, child := range p.cprocs {
		w := child.totalWeight()
		m[w] = append(m[w], child)
	}
	return m
}

func (p proc) String() string {
	uchild := p.unbalancedChild()

	s := fmt.Sprintf("%s: weight=%v balanced=%v", p.pid, p.totalWeight(), p.balanced())
	for _, child := range p.cprocs {
		s += fmt.Sprintf("\n  %s: weight=%v totalWeight=%v", child.pid, child.weight, child.totalWeight())
		if child == uchild {
			s += " *"
		}
	}
	return s
}

func test() {
	raw := []byte(`pbga (66)
	xhth (57)
	ebii (61)
	havc (66)
	ktlj (57)
	fwft (72) -> ktlj, cntj, xhth
	qoyq (66)
	padx (45) -> pbga, havc, qoyq
	tknk (41) -> ugml, padx, fwft
	jptl (61)
	ugml (68) -> gyxo, ebii, jptl
	gyxo (61)
	cntj (57)`)

	input, err := parseInput(bytes.NewReader(raw))
	if err != nil {
		log.Fatalln("parse input:", err)
	}

	// part 1
	if want, got := "tknk", p1(input); want != got {
		log.Fatalf("part 1: wanted=%v, got=%v", want, got)
	}

	if want, got := 60, p2(input); want != got {
		log.Fatalf("part 2: wanted=%v, got=%v", want, got)
	}
}

func p1(procs []proc) string {
	// make a mapping of pid -> ppid
	ps := make(map[string]string)
	for _, proc := range procs {
		if _, ok := ps[proc.pid]; !ok {
			ps[proc.pid] = ""
		}
		for _, child := range proc.children {
			ps[child] = proc.pid
		}
	}

	// find the pid with no ppid
	for pid, ppid := range ps {
		if ppid == "" {
			return pid
		}
	}

	return ""
}

func makeProcMap(procs []proc) map[string]proc {
	m := make(map[string]proc)
	for _, p := range procs {
		m[p.pid] = p
	}
	return m

}

func p2(procs []proc) int {
	// get the top pid by calling the function for the first part
	// of this puzzle
	rootPID := p1(procs)
	if rootPID == "" {
		return 0
	}

	ps := makeProcMap(procs)

	log.Println("building process tree")
	var root proc
	for _, p := range procs {
		if p.pid == rootPID {
			root = p
			break
		}
	}
	buildProcTree(&root, ps)

	return findIdealWeight(&root)
}

// perform a depth-first search of all procs, starting at p
func findIdealWeight(p *proc) int {
	log.Println(p)

	wg := p.groupChildrenByWeight()
	switch len(wg) {
	case 1:
		// balanced
		return 0

	case 2:
		for _, child := range p.cprocs {

		}
		return findIdealWeight(p.getUnbalancedChild())

	default:
		panic("unbalanced, but unexpected")
	}

	// get a random key from the map
	for w, _ := range wg {
		max = w
		break
	}
	for w, _ := range wg {
		min = w
		break
	}

	for w, children := range wg {
		if len(children) < len(wg[max]) {
			max = w
			continue
		}
		if len(children) > len(wg[min]) {
			min = w
		}
	}
	delta := max - min
	log.Printf("max=%v min=%v delta=%v", max, min, delta)
	return p.unbalancedChild().weight - delta
}

func buildProcTree(p *proc, ps map[string]proc) {
	for _, childPID := range p.children {
		child := ps[childPID]
		child.pproc = p

		p.cprocs = append(p.cprocs, &child)

		buildProcTree(&child, ps)
	}
}
