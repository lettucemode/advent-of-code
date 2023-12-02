package solutions

import (
	"bufio"
	"fmt"
	"io"
	"sort"
)

// D13Solve ...
func D13Solve(input io.Reader) (p1 interface{}, p2 interface{}) {

	// read input and build track
	carts := make([]*cart, 0)
	track := make(map[point]rune)
	scanner := bufio.NewScanner(input)
	lineCount := 0
	for scanner.Scan() {
		line := scanner.Text()
		for i, c := range line {
			if c == '^' {
				carts = append(carts, &cart{point{i, lineCount}, up, 0})
				track[point{i, lineCount}] = '|'
			} else if c == 'v' {
				carts = append(carts, &cart{point{i, lineCount}, down, 0})
				track[point{i, lineCount}] = '|'
			} else if c == '<' {
				carts = append(carts, &cart{point{i, lineCount}, left, 0})
				track[point{i, lineCount}] = '-'
			} else if c == '>' {
				carts = append(carts, &cart{point{i, lineCount}, right, 0})
				track[point{i, lineCount}] = '-'
			} else if c != ' ' {
				track[point{i, lineCount}] = c
			}
		}
		lineCount++
	}

	firstCrash := true
	for {
		sort.Slice(carts, func(x, y int) bool {
			return (carts[x].p.y)*1000+carts[x].p.x < (carts[y].p.y)*1000+carts[y].p.x
		})
		for i, cart := range carts {
			// move cart in direction
			if cart.dir == up {
				cart.p.y--
			} else if cart.dir == down {
				cart.p.y++
			} else if cart.dir == left {
				cart.p.x--
			} else if cart.dir == right {
				cart.p.x++
			}
			// change direction if necessary
			t := track[point{cart.p.x, cart.p.y}]
			if t == '/' {
				if cart.dir == up {
					cart.dir = right
				} else if cart.dir == down {
					cart.dir = left
				} else if cart.dir == left {
					cart.dir = down
				} else if cart.dir == right {
					cart.dir = up
				}
			} else if t == '\\' {
				if cart.dir == up {
					cart.dir = left
				} else if cart.dir == down {
					cart.dir = right
				} else if cart.dir == left {
					cart.dir = up
				} else if cart.dir == right {
					cart.dir = down
				}
			} else if t == '+' {
				if cart.turn%3 == 0 { // turn left
					if cart.dir == up {
						cart.dir = left
					} else if cart.dir == down {
						cart.dir = right
					} else if cart.dir == left {
						cart.dir = down
					} else if cart.dir == right {
						cart.dir = up
					}
				} else if cart.turn%3 == 2 { // turn right
					if cart.dir == up {
						cart.dir = right
					} else if cart.dir == down {
						cart.dir = left
					} else if cart.dir == left {
						cart.dir = up
					} else if cart.dir == right {
						cart.dir = down
					}
				}
				cart.turn++
			}
			// check for crash
			for k := range carts {
				if i == k {
					continue
				}
				if cart.p.x == carts[k].p.x && cart.p.y == carts[k].p.y {
					if firstCrash {
						p1 = fmt.Sprintf("%v,%v", cart.p.x, cart.p.y)
						firstCrash = false
					}
					goto breakout
				}
			}
		}
		// display := make([][]rune, 6)
		// for i := 0; i < 6; i++ {
		// 	display[i] = make([]rune, 13)
		// 	for k := 0; k < 13; k++ {
		// 		display[i][k] = ' '
		// 	}
		// }
		// for p, t := range track {
		// 	display[p.y][p.x] = t
		// }
		// for _, c := range carts {
		// 	if c.dir == up {
		// 		display[c.p.y][c.p.x] = '^'
		// 	} else if c.dir == down {
		// 		display[c.p.y][c.p.x] = 'v'
		// 	} else if c.dir == left {
		// 		display[c.p.y][c.p.x] = '<'
		// 	} else if c.dir == right {
		// 		display[c.p.y][c.p.x] = '>'
		// 	}
		// }
		// for _, s := range display {
		// 	fmt.Println(string(s))
		// }
	}
breakout:
	return
}

type direction int

const (
	up direction = iota
	down
	left
	right
)

type cart struct {
	p    point
	dir  direction
	turn int
}
