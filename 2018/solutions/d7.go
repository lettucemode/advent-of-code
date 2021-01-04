package solutions

import (
	"bufio"
	"fmt"
	"io"
	"regexp"
)

// D7Solve ...
func D7Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	nodes := make(map[string]*node)
	re := regexp.MustCompile(`([A-Z]) must be finished before step ([A-Z])`)
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		matches := re.FindStringSubmatch(scanner.Text())
		pn, cn := getNode(&nodes, matches[1]), getNode(&nodes, matches[2])
		pn.child = append(pn.child, cn)
		cn.parent = append(cn.parent, pn)
	}
	
	p1 = ""
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
		minKey := "Z"
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
		p1 = fmt.Sprintf("%v", p1) + next.key
	}

	// todo: part 2
	steps := fmt.Sprintf("%v", p1)
	p2 = 37

	return
}

func getNode(nodes *map[string]*node, key string) *node { 
	n, ok := (*nodes)[key]
	if !ok {
		n = newNode(key)
		(*nodes)[key] = n
	}
	return n
}

func newNode(key string) *node {
	return &node{key, make([]*node, 0), make([]*node, 0)}
}

type node struct {
	key string
	parent []*node
	child []*node
}