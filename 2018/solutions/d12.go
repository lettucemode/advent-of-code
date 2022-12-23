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
	pots := make([][]bool, num_generations+1)
	pots[0] = make([]bool, pots_size)
	rules := make(map[uint8]bool)
	var offset int
	for scanner.Scan() {
		line := scanner.Text()
		if strings.Contains(line, "initial") {
			start := strings.Index(line, "#")
			state := line[start:]
			offset = (pots_size - len(state)) / 2
			for i, c := range state {
				pots[0][i+offset] = c == '#'
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

	for g := 1; g < num_generations+1; g++ {
		fmt.Printf("%2v: ", g)
		pots[g] = make([]bool, pots_size)
		for i := 0; i < pots_size; i++ {
			if i < 2 {
				pots[g][i] = false
			} else if i > pots_size-3 {
				pots[g][i] = false
			} else {
				key := toKey(pots[g-1][i-2 : i+3])
				pots[g][i] = rules[key]
			}
		}
		for _, p := range pots[g] {
			c := "."
			if p {
				c = "#"
			}
			fmt.Print(c)
		}
		fmt.Println()
	}

	p1 = 0
	for i, p := range pots[num_generations] {
		if p {
			p1 = p1.(int) + (i - offset)
		}
	}

	return
}

func toKey(pots []bool) uint8 {
	key := uint8(0)
	for i, p := range pots {
		v := uint8(0)
		if p {
			v = 1
		}
		key |= v << i
	}
	return key
}
