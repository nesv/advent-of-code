package main

import (
	"bufio"
	"bytes"
	"strings"
	"testing"
)

func TestDecompress(t *testing.T) {
	for input, want := range map[string]string{
		"ADVENT":            "ADVENT",
		"A(1x5)BC":          "ABBBBBC",
		"(3x3)XYZ":          "XYZXYZXYZ",
		"A(2x2)BCD(2x2)EFG": "ABCBCDEFEFG",
		"(6x1)(1x3)A":       "(1x3)A",
		"X(8x2)(3x3)ABCY":   "X(3x3)ABC(3x3)ABCY",
	} {
		buf := new(bytes.Buffer)
		n, err := Decompress(buf, bufio.NewReader(strings.NewReader(input)))
		if err != nil {
			t.Error(err)
		}
		got := buf.String()
		t.Logf("%q => %q", input, got)
		if len(got) != n {
			t.Error("expected %d bytes, got %d", n, len(got))
		}
		if got != want {
			t.Errorf("wanted=%q got=%q", want, got)
		}
	}
}

func TestParseRepetition(t *testing.T) {
	for input, want := range map[string]struct{ chars, times int }{
		"1x5":    {1, 5},
		"3x3":    {3, 3},
		"(2x2)":  {2, 2},
		"(6x1":   {6, 1},
		")(3x3)": {3, 3},
	} {
		n, times, err := ParseRepetition(input)
		if err != nil {
			t.Error(err)
		}

		t.Logf("%q: chars=%d times=%d", input, n, times)

		if n != want.chars {
			t.Errorf("%q: wanted=%d got=%d", input, want.chars, n)
		}
		if times != want.times {
			t.Errorf("%q: wanted=%d got=%d", input, want.times, times)
		}
	}
}
