package main

import (
	"io"
	"strings"
	"testing"
)

var testRules = `light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.`

func TestRulesContains(t *testing.T) {
	rules, err := rulesFrom(strings.NewReader(testRules))
	if err != nil {
		t.Fatal(err)
	}

	bag := "shiny gold"
	bags := rules.contains(bag)
	if want, got := 4, len(bags); want != got {
		t.Errorf("%s: want=%d got=%d", bag, want, got)
	}
}

func TestRulesNbags(t *testing.T) {
	tests := []struct {
		r    io.Reader
		bag  string
		want int
	}{
		{
			r:    strings.NewReader(testRules),
			bag:  "shiny gold",
			want: 32,
		},
		{
			r: strings.NewReader(`shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.`),
			bag:  "shiny gold",
			want: 126,
		},
	}

	for _, tt := range tests {
		rules, err := rulesFrom(tt.r)
		if err != nil {
			t.Fatal(err)
		}

		if want, got := tt.want, rules.nbags(tt.bag); want != got {
			t.Errorf("number of bags to purchase: %s: want=%d got=%d", tt.bag, want, got)
		}
	}
}
