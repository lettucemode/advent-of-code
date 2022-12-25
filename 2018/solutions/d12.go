package solutions

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

// D12Solve ...
func D12Solve(input io.Reader) (p1 interface{}, p2 interface{}) {

	const pots_size int = 200
	const num_generations int = 20

	scanner := bufio.NewScanner(input)
	pots := make(map[int]bool)
	rules := make(map[uint8]bool)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.Contains(line, "initial") {
			start := strings.Index(line, "#")
			state := line[start:]
			for i, c := range state {
				pots[i] = c == '#'
			}
		} else if strings.Contains(line, "=>") {
			parts := strings.Split(line, " => ")
			var key uint8
			for i, c := range parts[0] {
				v := uint8(0)
				if c == '#' {
					v = 1
				}
				key |= v << i
			}
			rules[key] = parts[1][0] == '#'
		}
	}

	next_gen_pots := make(map[int]bool)
	printPotGen(pots, 0)
	for g := 0; g < num_generations; g++ {
		for i := -5; i < 205; i++ {
			key := toKey(pots, i)
			next_gen_pots[i] = rules[key]
		}

		printPotGen(next_gen_pots, g+1)
		for k := range pots {
			delete(pots, k)
			pots[k] = next_gen_pots[k]
		}
	}

	p1 = 0
	for i, p := range pots {
		if p {
			p1 = p1.(int) + i
		}
	}

	return
}

func printPotGen(pots map[int]bool, gen int) {
	fmt.Printf("%3v: ", gen)
	row := make([]rune, 210)
	for i := -5; i < 205; i++ {
		c := '.'
		pot, ok := pots[i]
		if ok && pot {
			c = '#'
		}
		row[i+5] = c
	}
	fmt.Println(string(row))
}

func toKey(pots map[int]bool, index int) uint8 {
	key := uint8(0)
	for i, k := index-2, 0; i < index+3; i, k = i+1, k+1 {
		v := uint8(0)
		pot, ok := pots[i]
		if ok && pot {
			v = 1
		}
		key |= v << k
	}
	return key
}
