package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

type Node struct {
	left  string
	right string
}

func main() {
	file, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	start := time.Now()

	directions, nodes := parse_input(scanner)

	var starting_nodes []int

	for k := range nodes {
		if strings.HasSuffix(k, "A") {
			starting_nodes = append(starting_nodes, walk_directions(nodes, 0, directions, k, 0))
		}
	}

	lcm := lcm(starting_nodes)

	time_took := time.Since(start)

	fmt.Println("Result:", lcm)

	fmt.Println("Took", time_took)
}

// least common multiplier
func lcm(n []int) int {
	res := n[0]
	for _, v := range n {
		res = res * v / gcd(res, v)
	}
	return res
}

// greatest commom denominator
func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func parse_input(scanner *bufio.Scanner) ([]rune, map[string]Node) {
	var lines []string
	var directions []rune
	nodes := make(map[string]Node)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	directions = []rune(lines[0])

	for _, line := range lines[2:] {
		var source, left, right string
		fmt.Sscanf(line, "%s = (%3s, %3s)", &source, &left, &right)
		nodes[source] = Node{left, right}
	}
	return directions, nodes
}

func walk_directions(nodes map[string]Node, step int, directions []rune, location string, counter int) int {
	if strings.HasSuffix(location, "Z") {
		return counter
	}

	if step == len(directions) {
		step = 0
	}

	if directions[step] == 'L' {
		return walk_directions(nodes, step+1, directions, nodes[location].left, counter+1)
	} else {
		return walk_directions(nodes, step+1, directions, nodes[location].right, counter+1)
	}
}
