package solutions

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

// D12Solve ...
func D12Solve(input io.Reader) (p1 interface{}, p2 interface{}) {

	// setup for p1
	const pots_size int = 200
	const num_generations int = 20

	// parse input & build rules
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

	// run p1 simulation
	min_pot, max_pot := 0, 95
	g := 0
	// printPotGen(pots, 0, min_pot, max_pot)
	for ; g < num_generations; g++ {
		advanceGeneration(&pots, &rules, &min_pot, &max_pot)
		// printPotGen(pots, g+1, min_pot, max_pot)
	}
	p1 = calcAnswer(pots)

	// So now we need to change tack - 50B generations is too much.
	// So instead let's look when the pot-number-calc value
	// is the same difference from the previous generation
	// for several gens in a row.
	lastTenAnswers := make([]int, 0)
	lastTenAnswers = append(lastTenAnswers, p1.(int))
	allSameDiffs := false
	for {
		advanceGeneration(&pots, &rules, &min_pot, &max_pot)
		lastTenAnswers = append(lastTenAnswers, calcAnswer(pots))
		g++
		if len(lastTenAnswers) > 10 {
			lastTenAnswers = lastTenAnswers[1:]
		}
		if len(lastTenAnswers) == 10 {
			allSameDiffs = true
			firstDiff := lastTenAnswers[1] - lastTenAnswers[0]
			for i := 2; i < 9; i++ {
				allSameDiffs = allSameDiffs && ((lastTenAnswers[i] - lastTenAnswers[i-1]) == firstDiff)
			}
		}
		if allSameDiffs {
			break
		}
	}
	diff := lastTenAnswers[len(lastTenAnswers)-1] - lastTenAnswers[len(lastTenAnswers)-2]
	fmt.Println(g, diff)

	return
}

func advanceGeneration(pots *map[int]bool, rules *map[uint8]bool, min_pot *int, max_pot *int) {
	next_gen_pots := make(map[int]bool)
	for i := *min_pot - 4; i < *max_pot+5; i++ {
		key := toKey(*pots, i)
		next_gen_pots[i] = (*rules)[key]
		if next_gen_pots[i] {
			if i < *min_pot {
				*min_pot = i
			} else if i > *max_pot {
				*max_pot = i
			}
		}
	}

	for k := range *pots {
		delete(*pots, k)
	}
	for k := range next_gen_pots {
		(*pots)[k] = next_gen_pots[k]
	}
}

func printPotGen(pots map[int]bool, gen int, min_pot int, max_pot int) {
	fmt.Printf("%3v: ", gen)
	row := make([]rune, max_pot+5)
	for i := min_pot; i < max_pot; i++ {
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

func calcAnswer(pots map[int]bool) (total int) {
	total = 0
	for i, p := range pots {
		if p {
			total += i
		}
	}
	return
}
