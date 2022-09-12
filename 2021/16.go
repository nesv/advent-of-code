package main

import (
	_ "embed"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

//go:embed input/16.test
var input string

func main() {
	input := strings.TrimSpace(input)
	fmt.Printf("%s\t%s\n", input, hex2bin(input))

	packets, err := parsePackets(hex2bin(input))
	if err != nil {
		log.Fatalln("parse packets:", err)
	}

	for _, p := range packets {
		fmt.Println(p)
	}

	// Part 1: What do you get if you add up the version numbers in all packets?
	fmt.Println(part1(input))
}

func parsePackets(input string) ([]packet, error) {
	var (
		pp    []packet
		start int
	)
	for {
		var p packet
		n, err := p.parse(input[start:])
		if err != nil {
			return nil, err
		}
		if n == 0 {
			break
		}

		pp = append(pp, p)

		start += n
	}
	return pp, nil
}

type packet struct {
	version uint8
	typeID  uint8
	groups  []uint8
	packets []packet
}

func (p *packet) parse(input string) (int, error) {
	if input == "" {
		return 0, nil
	}

	var n int

	// Read the packet version (3 bits).
	v, err := strconv.ParseUint(input[n:n+3], 2, 8)
	if err != nil {
		return n, err
	}
	p.version = uint8(v)
	n += 3

	// Read the packet type ID (3 bits).
	t, err := strconv.ParseUint(input[n:n+3], 2, 8)
	if err != nil {
		return n, err
	}
	p.typeID = uint8(t)
	n += 3

	if p.typeID == 4 {
		// Keep reading chunks of 5 until you find a 5-bit chunk that
		// starts with a "0".
		for {
			chunk := input[n : n+5]
			v, err := strconv.ParseUint(chunk[1:], 2, 8)
			if err != nil {
				return n, err
			}

			p.groups = append(p.groups, uint8(v))
			n += 5

			if chunk[0] == '0' {
				break
			}
		}
	} else {
		// Operator packet.
		// Read the length type ID.
		switch input[n] {
		case '0':
			// The next 15 bits are a number that represents the total
			// length in bits of the sub-packets contained by this packet.
			n++

			nbits, err := strconv.ParseUint(input[n:n+15], 2, 16)
			if err != nil {
				return n, err
			}
			n += 15

			fmt.Fprintln(os.Stderr, "nbits =", nbits)

			// Parse sub-packets.
			subpackets, err := parsePackets(input[n : n+int(nbits)])
			if err != nil {
				return n, fmt.Errorf("parse sub-packets: %w", err)
			}
			p.packets = subpackets

			n += int(nbits)

		case '1':
			// The next 11 bits are a number that represents the number
			// of sub-packets immediately contained by this packet.
			n++

			np, err := strconv.ParseUint(input[n:n+11], 2, 16)
			if err != nil {
				return n, err
			}
			n += 11

			// Parse "np" subpackets.
			for i := np; i >= 0; i-- {
				var sp packet
				m, err := sp.parse(input[n:])
				if err != nil {
					return n, fmt.Errorf("parse sub-packet: %w", err)
				}
				n += m
				p.packets = append(p.packets, sp)
			}
		}
	}

	// Pad "n" until we get to a multiple of 4.
	if m := n % 4; n != 0 {
		n += 4 - m
	}

	return n, nil
}

func (p packet) String() string {
	var groups []string
	for _, g := range p.groups {
		groups = append(groups, fmt.Sprintf("%d", g))
	}

	return fmt.Sprintf("%d:%d:%s", p.version, p.typeID, strings.Join(groups, ","))
}

func part1(input string) int {
	return 0
}

func hex2bin(input string) string {
	var b strings.Builder
	b.Grow(len(input) * 4)
	for _, v := range input {
		switch v {
		case '0':
			if _, err := b.WriteString("0000"); err != nil {
				panic(err)
			}
		case '1':
			if _, err := b.WriteString("0001"); err != nil {
				panic(err)
			}
		case '2':
			if _, err := b.WriteString("0010"); err != nil {
				panic(err)
			}
		case '3':
			if _, err := b.WriteString("0011"); err != nil {
				panic(err)
			}
		case '4':
			if _, err := b.WriteString("0100"); err != nil {
				panic(err)
			}
		case '5':
			if _, err := b.WriteString("0101"); err != nil {
				panic(err)
			}
		case '6':
			if _, err := b.WriteString("0110"); err != nil {
				panic(err)
			}
		case '7':
			if _, err := b.WriteString("0111"); err != nil {
				panic(err)
			}
		case '8':
			if _, err := b.WriteString("1000"); err != nil {
				panic(err)
			}
		case '9':
			if _, err := b.WriteString("1001"); err != nil {
				panic(err)
			}
		case 'A':
			if _, err := b.WriteString("1010"); err != nil {
				panic(err)
			}
		case 'B':
			if _, err := b.WriteString("1011"); err != nil {
				panic(err)
			}
		case 'C':
			if _, err := b.WriteString("1100"); err != nil {
				panic(err)
			}
		case 'D':
			if _, err := b.WriteString("1101"); err != nil {
				panic(err)
			}
		case 'E':
			if _, err := b.WriteString("1110"); err != nil {
				panic(err)
			}
		case 'F':
			if _, err := b.WriteString("1111"); err != nil {
				panic(err)
			}
		default:
			panic(fmt.Sprintf("weird value: %q", string(v)))
		}
	}
	return b.String()
}
