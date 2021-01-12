package solutions

import (
	"bufio"
	"fmt"
	"io"
	"regexp"
	"sort"
)

// D7Solve ...
func D7Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	lines := make([]string, 0)
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	
	p1 = partOne(lines)
	p2 = partTwo(lines)
	return
}

func partTwo(lines []string) int {

	nodes := make(map[rune]*node)
	re := regexp.MustCompile(`([A-Z]) must be finished before step ([A-Z])`)
	for _, line := range lines {
		matches := re.FindStringSubmatch(line)
		pn, cn := getNode(&nodes, toRune(matches[1])), getNode(&nodes, toRune(matches[2]))
		pn.child = append(pn.child, cn)
		cn.parent = append(cn.parent, pn)
	}

	totalTime := 0
	for len(nodes) > 0 {

		// find available
		avail := make([]*node, 0)
		for _, n := range nodes {
			if len(n.parent) == 0 {
				avail = append(avail, n)
			}
		}

		// if none available, we're done
		if len(avail) == 0 {
			break
		}

		// sort em
		keys := make([]int, 0)
		for _, n := range avail {
			keys = append(keys, int(n.key))
		}
		sort.Ints(keys)

		// decrement up to the first five
		for i := 0; i < 5 && i < len(keys); i++ {
			n := nodes[rune(keys[i])]
			n.duration--

			// if duration is now 0, remove it
			if n.duration == 0 {
				for _, c := range n.child {
					for j, p := range c.parent {
						if p == n {
							c.parent[j] = c.parent[len(c.parent) - 1]
							c.parent = c.parent[:len(c.parent) - 1]
							break
						}
					}
				}
				delete(nodes, n.key)
			}
		}
		totalTime++
	}
	return totalTime
}

func partOne(lines []string) string {

	nodes := make(map[rune]*node)
	re := regexp.MustCompile(`([A-Z]) must be finished before step ([A-Z])`)
	for _, line := range lines {
		matches := re.FindStringSubmatch(line)
		pn, cn := getNode(&nodes, toRune(matches[1])), getNode(&nodes, toRune(matches[2]))
		pn.child = append(pn.child, cn)
		cn.parent = append(cn.parent, pn)
	}

	ret := ""
	for len(nodes) > 0 {

		// find available
		avail := make([]*node, 0)
		for _, n := range nodes {
			if len(n.parent) == 0 {
				avail = append(avail, n)
			}
		}

		// if none available, we're done
		if len(avail) == 0 {
			break
		}

		// get the first alphabetically
		var next *node
		minKey := 'Z'
		for _, n := range avail {
			if n.key <= minKey {
				next = n
				minKey = n.key
			}
		}

		// remove it from its children and from nodes
		for _, c := range next.child {
			for j, p := range c.parent {
				if p == next {
					c.parent[j] = c.parent[len(c.parent) - 1]
					c.parent = c.parent[:len(c.parent) - 1]
					break
				}
			}
		}
		delete(nodes, next.key)
		ret = fmt.Sprintf("%v", ret) + string(next.key)
	}
	return ret
}

func toRune(in string) rune {
	tmp := []rune(in)
	if len(tmp) > 1 {
		panic("length of in param greater than one!!!")
	}
	return tmp[0]
}

func getNode(nodes *map[rune]*node, key rune) *node { 
	n, ok := (*nodes)[key]
	if !ok {
		n = newNode(key)
		(*nodes)[key] = n
	}
	return n
}

func newNode(key rune) *node {
	return &node{key, make([]*node, 0), make([]*node, 0), 60 + int(key) - 64}
}

type node struct {
	key rune
	parent []*node
	child []*node
	duration int
}