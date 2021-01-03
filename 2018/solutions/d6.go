package solutions

import (
	"bufio"
	"io"
	"math"
	"strconv"
	"strings"
)

// D6Solve ...
func D6Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	points := makePoints(input)	
	scores := make([]int, len(*points))
	dists := make([]int, len(*points))
	safeSize := 0

	minX, minY, maxX, maxY := 10000, 10000, 0, 0
	for _, p := range *points {
		if p.x < minX {
			minX = p.x
		} else if p.x > maxX {
			maxX = p.x
		}
		if p.y < minY {
			minY = p.y
		} else if p.y > maxY {
			maxY = p.y
		}
	}

	for x := minX; x <= maxX; x++ {
		for y := minY; y <= maxY; y++ {
			for pi, pt := range *points {
				dists[pi] = manhattan(point{x, y}, pt)
			}
			if sum(&dists) < 10000 {
				safeSize++
			}
			mindex, _, tied := min(&dists)
			if !tied {
				scores[mindex]++
			}
		}
	}
	_, p1 = max(&scores)
	p2 = safeSize
	return
}

func sum(list *[]int) (sum int) {
	for _, v := range *list {
		sum += v
	}
	return
}

func max(list *[]int) (int, int) {
	mindex, mval := 0, 0
	for k, v := range *list {
		if v > mval {
			mval = v
			mindex = k
		}
	}
	return mindex, mval
}

func min(list *[]int) (int, int, bool) {
	mindex, mval, tied := 0, math.MaxInt32, false
	for k, v := range *list {
		if v < mval {
			mval = v
			mindex = k
		} 
	}
	for k, v := range *list {
		if v == mval && k != mindex {
			tied = true
			break
		}
	}
	return mindex, mval, tied
}

func makePoints(input io.Reader) *[]point {
	points := make([]point, 0)
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		parts := strings.Split(scanner.Text(), ", ")
		x, _ := strconv.Atoi(parts[0])
		y, _ := strconv.Atoi(parts[1])
		points = append(points, point{x, y})
	}
	return &points
}

func manhattan(p1, p2 point) int {
	return abs(p1.x - p2.x) + abs(p1.y - p2.y)
}

func abs(v int) int {
	if v < 0 {
		return -v
	}
	return v
}

type point struct {
	x int
	y int
}