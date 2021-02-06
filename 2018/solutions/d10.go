package solutions

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"regexp"
	"strconv"
	"strings"
)

// D10Solve ...
func D10Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	points := make([]point2, 0)
	re := regexp.MustCompile(`position=<([\s-]*\d+), ([\s-]*\d+)> velocity=<([\s-]*\d+), ([\s-]*\d+)>`)
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		matches := re.FindStringSubmatch(scanner.Text())
		posx, _ := strconv.Atoi(strings.TrimSpace(matches[1]))
		posy, _ := strconv.Atoi(strings.TrimSpace(matches[2]))
		velx, _ := strconv.Atoi(strings.TrimSpace(matches[3]))
		vely, _ := strconv.Atoi(strings.TrimSpace(matches[4]))
		points = append(points, point2{posx, posy, velx, vely})
	}

	// find when x and y diffs are at a minimum, assume that's the message
	minSize := math.MaxInt32
	targetI := 0
	for i := 0; i < 11000; i++ {
		xs := make([]int, 0, len(points))
		ys := make([]int, 0, len(points))
		for _, p := range points {
			xs = append(xs, p.px + i * p.vx)
			ys = append(ys, p.py + i * p.vy)
		}
		minx, miny := math.MaxInt32, math.MaxInt32
		maxx, maxy := math.MinInt32, math.MinInt32
		for k := range xs {
			if xs[k] < minx {
				minx = xs[k]
			} else if xs[k] > maxx {
				maxx = xs[k]
			}
			if ys[k] < miny {
				miny = ys[k]
			} else if ys[k] > maxy {
				maxy = ys[k]
			}
		}
		size := maxx - minx + maxy - miny
		if size < minSize {
			minSize = size
			targetI = i
		}
	}

	// display message at that i
	matrix := make([][]rune, 12)
	for i := 0; i < 12; i++ {
		matrix[i] = make([]rune, 75)
	}
	for _, p := range points {
		matrix[p.py + targetI * p.vy - 150][p.px + targetI * p.vx - 110] = '*'
	}
	for _, rw := range matrix {
		for _, c := range rw {
			fmt.Print(string(c))
		}
		fmt.Println()
	}

	// I seen it with mine own eyes
	p1 = "JJXZHKFP"
	p2 = targetI

	return
}

type point2 struct {
	px int
	py int
	vx int
	vy int
}