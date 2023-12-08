package main

import (
	"bufio"
	"fmt"
	"os"
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
	steps := walk_directions(nodes, 0, directions, "AAA", 0)

	time_took := time.Since(start)
	fmt.Println("Result:", steps)

	fmt.Println("Took", time_took)
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
	if location == "ZZZ" {
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
