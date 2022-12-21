package solutions

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"math"
	"strconv"
)

const gridSize int = 300

// D11Solve ...
func D11Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	scanner := bufio.NewScanner(input)
	scanner.Scan()
	serialNo, err := strconv.Atoi(scanner.Text())
	if err != nil {
		log.Fatal(err)
	}

	// calculate cells with one goroutine per row
	// for cool factor
	row := make([]int, 0, gridSize)
	for k := 1; k <= gridSize; k++ {
		row = append(row, k)
	}
	ch := make(chan *powerCell)
	for y := 1; y <= gridSize; y++ {
		y := y
		go calcPowerForRow(&row, y, serialNo, ch)
	}
	cells := make(map[key]int)
	for j := 0; j < gridSize*gridSize; j++ {
		c := <-ch
		cells[key{c.x, c.y}] = c.powerLevel
	}

	// After much deliberation and experimentation, I looked up
	// that there is such a thing as a sum table. So let's use that.
	sumTable := make([][]int, gridSize+1)
	for i := 0; i < gridSize+1; i++ {
		sumTable[i] = make([]int, gridSize+1)
	}

	bestX, bestY, bestS, bestP := math.MinInt, math.MinInt, math.MinInt, math.MinInt
	for x := 1; x <= gridSize; x++ {
		for y := 1; y <= gridSize; y++ {
			p := cells[key{x, y}]
			sumTable[x][y] = p + sumTable[x-1][y] + sumTable[x][y-1] - sumTable[x-1][y-1]
		}
	}
	for s := 1; s <= gridSize; s++ {
		for x := 1; x < gridSize-s+1; x++ {
			for y := 1; y < gridSize-s+1; y++ {
				total := sumTable[x][y] - sumTable[x+s][y] - sumTable[x][y+s] + sumTable[x+s][y+s]
				if total > bestP {
					bestP = total
					bestX = x + 1
					bestY = y + 1
					bestS = s
				}
			}
		}
		if s == 3 {
			p1 = fmt.Sprintf("%v,%v", bestX, bestY)
		}
	}
	p2 = fmt.Sprintf("%v,%v,%v", bestX, bestY, bestS)
	return
}

func calcPowerForRow(xs *[]int, y int, serialNo int, ch chan<- *powerCell) {
	for _, x := range *xs {
		rackID := x + 10
		powerLevel := rackID * y
		powerLevel += serialNo
		powerLevel *= rackID
		powerLevel = (powerLevel / 100) % 10
		powerLevel -= 5
		ch <- &powerCell{x, y, powerLevel}
	}
}

type key struct {
	x, y int
}

type powerCell struct {
	x, y, powerLevel int
}
