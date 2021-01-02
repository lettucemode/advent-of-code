package solutions

import (
	"bufio"
	"io"
	"math"
)

const asciiStart int = 65;

// D5Solve ...
func D5Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	scanner := bufio.NewScanner(input)
	scanner.Scan()
	bs := []byte(scanner.Text())
	bs = react(bs)	
	p1 = len(bs)

	bs = []byte(scanner.Text())
	minLength := math.MaxInt32
	ch := make(chan int)

	// this is cool. could maybe be faster?
	// great for now though. first version took 30s for one react()
	for a := 0; a < 26; a++ {
		copy := append([]byte(nil), bs...)
		go reduceAndReact(copy, a, ch)
	}
	for x := 0; x < 26; x++ {
		v := <- ch
		if v < minLength {
			minLength = v
		}
	}
	p2 = minLength
	return
}

func reduceAndReact(bs []byte, a int, ch chan int) {
	n := 0
	for _, b := range bs {
		if b != byte(asciiStart + a) && b != byte(asciiStart + a + 32) {
			bs[n] = b
			n++
		}
	}
	bs = bs[:n]

	bs = react(bs)
	ch <- len(bs)
}

func react(bs []byte) []byte {
	for {
		removed := false
		for i := 0; i < len(bs) - 1; i++ {
			diff := int(bs[i]) - int(bs[i + 1])
			if diff == 32 || diff == -32 {
				bs = append(bs[:i], bs[i+2:]...)
				i--
				removed = true
				continue
			}
		}
		if !removed {
			break
		}
	}
	return bs
}