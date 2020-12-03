package solutions

import (
	"bufio"
	"io"
	"regexp"
	"strconv"
)

// D3Solve ...
func D3Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	fabric := makeFabric(1000)
	claims := parseClaims(input)
	for _, claim := range claims {
		for i := 0; i < claim.H; i++ {
			for j := 0; j < claim.W; j++ {
				fabric[claim.Y + i][claim.X + j]++
			}
		}
	}
	
	
	totalOverlaps := 0
	for ii := range fabric {
		for jj := range fabric[ii] {
			if 2 <= fabric[ii][jj] {
				totalOverlaps++
			}
		}
	}
	p1 = totalOverlaps

	for _, claim := range claims {
		allOnes := true
		for i := 0; i < claim.H; i++ {
			for j := 0; j < claim.W; j++ {
				if 2 <= fabric[claim.Y + i][claim.X + j] {
					allOnes = false
				}
			}
		}
		if allOnes {
			p2 = claim.Elf
		}
	}
	return
}

func makeFabric(size int) [][]int {
	fabric := make([][]int, size)
	for ii := range fabric {
		fabric[ii] = make([]int, size)
	}
	return fabric
}

func parseClaims(input io.Reader) []Claim {
	claims := make([]Claim, 0, 1357)
	re := regexp.MustCompile(`#(\d+) @ (\d+),(\d+): (\d+)x(\d+)`)
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		line := scanner.Text()
		matches := re.FindStringSubmatch(line)
		elf, _ := strconv.Atoi(matches[1])
		x, _ := strconv.Atoi(matches[2])
		y, _ := strconv.Atoi(matches[3])
		w, _ := strconv.Atoi(matches[4])
		h, _ := strconv.Atoi(matches[5])
		claims = append(claims, Claim{elf, x, y, w, h})
	}
	return claims
}

// Claim ...
type Claim struct {
	Elf int
	X int
	Y int
	W int
	H int
}