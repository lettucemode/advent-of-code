package solutions

import (
	"bufio"
	"container/ring"
	"io"
	"regexp"
	"strconv"
)

// D9Solve ...
func D9Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	scanner := bufio.NewScanner(input)
	scanner.Scan()
	re := regexp.MustCompile(`(\d+)[a-zA-Z ;]+(\d+)`)
	matches := re.FindStringSubmatch(scanner.Text())
	players, _ := strconv.Atoi(matches[1])
	lastMarble, _ := strconv.Atoi(matches[2])

	p1 = findMaxScore(players, lastMarble)
	p2 = findMaxScore(players, lastMarble * 100)
	
	return
}

func findMaxScore(players int, lastMarble int) (maxScore int) {
	scores := make([]int, players, players)
	marble, player := 0, 0
	circle := ring.New(1)
	circle.Value = marble
	marble++

	for {
		if marble % 23 != 0 {		
			// insert new marble after next one
			circle = circle.Next()
			newMarble := ring.New(1)
			newMarble.Value = marble
			circle.Link(newMarble)
			circle = circle.Next()
		} else {
			// do scoring
			scores[player] += marble
			circle = circle.Move(-8)
			rem := circle.Link(circle.Next().Next())
			scores[player] += rem.Value.(int)
			circle = circle.Next()
		}

		// advance, quit if too high
		marble++
		if marble > lastMarble {
			break
		}
		player = (player + 1) % players
	}
	
	for _, score := range scores {
		if score > maxScore {
			maxScore = score
		}
	}
	return
}