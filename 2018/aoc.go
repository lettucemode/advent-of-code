package main

import (
	"aoc/inputs"
	"aoc/solutions"
	"fmt"
	"io"
	"time"
)

type puzzleFunc func(io.Reader) (interface{}, interface{})

func runPuzzle(day int, f puzzleFunc) {
	in := inputs.GetInput(day)
	start := time.Now()
	p1, p2 := f(in)
	elapsed := time.Now().Sub(start)
	fmt.Printf("Day %v: \n", day)
	fmt.Printf("\tPart 1: %v", fmt.Sprintf("%v", p1))
	fmt.Printf("\tPart 2: %v", fmt.Sprintf("%v", p2))
	fmt.Printf("\tDuration: %v\n", time.Duration(elapsed))
}

func main() {
	runPuzzle(1, puzzleFunc(solutions.D1Solve));
	runPuzzle(2, puzzleFunc(solutions.D2Solve));
	runPuzzle(3, puzzleFunc(solutions.D3Solve));
}
