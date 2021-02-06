package main

import (
	"aoc/inputs"
	"aoc/solutions"
	"fmt"
	"io"
	"time"
)

type puzzleFunc func(io.Reader) (interface{}, interface{})

func runPuzzle(day int, f puzzleFunc) (elapsed time.Duration) {
	in := inputs.GetInput(day)
	start := time.Now()
	p1, p2 := f(in)
	elapsed = time.Now().Sub(start)
	fmt.Printf("Day %v: \n", day)
	fmt.Printf("\tPart 1: %v", fmt.Sprintf("%v", p1))
	fmt.Printf("\tPart 2: %v", fmt.Sprintf("%v", p2))
	fmt.Printf("\tDuration: %v", elapsed)
	fmt.Println()
	return
}

func main() {
	var totalTime time.Duration
	totalTime += runPuzzle(1, puzzleFunc(solutions.D1Solve));
	totalTime += runPuzzle(2, puzzleFunc(solutions.D2Solve));
	totalTime += runPuzzle(3, puzzleFunc(solutions.D3Solve));
	totalTime += runPuzzle(4, puzzleFunc(solutions.D4Solve));
	totalTime += runPuzzle(5, puzzleFunc(solutions.D5Solve));
	totalTime += runPuzzle(6, puzzleFunc(solutions.D6Solve));
	totalTime += runPuzzle(7, puzzleFunc(solutions.D7Solve));
	totalTime += runPuzzle(8, puzzleFunc(solutions.D8Solve));
	totalTime += runPuzzle(9, puzzleFunc(solutions.D9Solve));
	totalTime += runPuzzle(10, puzzleFunc(solutions.D10Solve));

	fmt.Println()
	fmt.Println("Total time:")
	fmt.Printf("\t%v", totalTime)
}
