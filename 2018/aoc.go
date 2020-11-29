package main

import (
	"aoc/inputs"
	"aoc/solutions"
	"fmt"
	"time"
)

type puzzleFunc func(string) (interface{}, interface{})

func runPuzzle(day int, f puzzleFunc) {
	in := inputs.GetInput(day)
	start := time.Now()
	p1, p2 := f(in)
	p1ans := fmt.Sprintf("%v", p1)
	p2ans := fmt.Sprintf("%v", p2)
	elapsed := time.Now().Sub(start)
	fmt.Printf("Day %v: \n", day)
	fmt.Printf("\tPart 1: %v", p1ans)
	fmt.Printf("\tPart 2: %v", p2ans)
	fmt.Printf("\tDuration: %v", time.Duration(elapsed))
}

func main() {
	runPuzzle(1, puzzleFunc(solutions.D1Solve));
}
