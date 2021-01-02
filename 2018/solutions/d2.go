package solutions

import (
	"bufio"
	"io"
)

// D2Solve ...
func D2Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	var twos, threes int = 0, 0
	scanner := bufio.NewScanner(input)
	lines := make([]string, 0, 250)

	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
		chCounts := make(map[rune]int)
		for _, ch := range line {
			c, ok := chCounts[ch]
			if ok {
				chCounts[ch] = c + 1
			} else {
				chCounts[ch] = 1
			}
		}
		var hasTwo, hasThree bool = false, false
		for _, count := range chCounts {
			hasTwo = hasTwo || count == 2
			hasThree = hasThree || count == 3
		}
		if hasTwo {
			twos++
		}
		if hasThree {
			threes++
		}
	}
	p1 = twos * threes

	ii, index := 0, 0
	OuterLoop:
	for i, line1 := range lines {
		for j, line2 := range lines {
			if i == j {
				continue
			}
			diff := 0
			diffIndex := 0
			for k := 0; k < 26; k++ {
				if line1[k] != line2[k] {
					diff++
					diffIndex = k
				}
			}
			if diff == 1 {
				ii = i
				index = diffIndex
				break OuterLoop
			}
		}
	}
	p2 = lines[ii][:index] + lines[ii][index+1:]
	return
}