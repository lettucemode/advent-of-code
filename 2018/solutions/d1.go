package solutions

import (
	"bufio"
	"io"
	"strconv"
	"strings"
)

// D1Solve ...
func D1Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	// since input is a reader, need to make a string from it
	// then create a new reader against that string each loop
	buf := new(strings.Builder)
	io.Copy(buf, input)
	inputStr := buf.String()

	var freq int = 0 
	pastFreqs := make(map[int]struct{})
	first := true
	for p2 == nil {
		scanner := bufio.NewScanner(strings.NewReader(inputStr))
		for scanner.Scan() {
			val, _ := strconv.Atoi(scanner.Text())
			freq += val
			_, exists := pastFreqs[freq]
			if exists && p2 == nil {
				p2 = freq
			}
			pastFreqs[freq] = struct{}{}
		}
		if first {
			p1 = freq
			first = false
		}
	}
	return
}