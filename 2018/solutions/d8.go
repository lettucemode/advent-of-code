package solutions

import (
	"bufio"
	"io"
	"strconv"
)

// D8Solve ...
func D8Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	nodes := make([]int, 0)
	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanWords)
	for scanner.Scan() {
		i, _ := strconv.Atoi(scanner.Text())
		nodes = append(nodes, i)
	}

	p1, _ = sumMetadata(nodes, 0)
	p2, _ = getNodeValue(nodes, 0)

	return
}

func sumMetadata(nodes []int, i int) (int, int) {
	numChildren, numMetadatas := nodes[i], nodes[i + 1]
	start, sum, k := i, 0, i + 2
	for c := 0; c < numChildren; c++ {
		s, len := sumMetadata(nodes, k)
		sum += s
		k += len
	}
	for m := 0; m < numMetadatas; m++ {
		sum += nodes[k]
		k++
	}
	return sum, k - start
}

func getNodeValue(nodes []int, i int) (int, int) {
	numChildren, numMetadatas := nodes[i], nodes[i + 1]
	start, sum, k := i, 0, i + 2
	if numChildren > 0 {
		childSums := make([]int, 0)
		childSums = append(childSums, 0)
		for c := 0; c < numChildren; c++ {
			s, len := getNodeValue(nodes, k)
			childSums = append(childSums, s)
			k += len
		}
		for m := 0; m < numMetadatas; m++ {
			if nodes[k] < len(childSums) {
				sum += childSums[nodes[k]]
			}
			k++
		}
	} else {
		for m := 0; m < numMetadatas; m++ {
			sum += nodes[k]
			k++
		}
	}
	return sum, k - start
}