package main

import (
	"bufio"
	"fmt"
	"os"
	"time"
)

const UP = 0
const DOWN = 1
const LEFT = 2
const RIGHT = 3

type Position struct {
	x int
	y int
}

func main() {
	file, _ := os.Open("input.txt")

	defer file.Close()

	scanner := bufio.NewScanner(file)

	input := parse_input(scanner)

	start := time.Now()

	seen := make(map[Position]int)
	seen[Position{0, 0}] = 1
	walk(&input, Position{0, 0}, Position{1, 0}, &seen)

	time_took := time.Since(start)
	fmt.Println("Result:", len(seen))

	fmt.Println("Took", time_took)
}

func parse_input(scanner *bufio.Scanner) [][]rune {
	var temp [][]rune

	for scanner.Scan() {
		temp = append(temp, []rune(scanner.Text()))
	}

	return temp
}

func walk(arr *[][]rune, curr, direction Position, seen *map[Position]int) {
	curr_direction := get_direction(direction)

	if curr.x == len((*arr)[0]) || curr.y == len(*arr) || curr.x == -1 || curr.y == -1 {
		return
	}

	(*seen)[curr]++

	switch (*arr)[curr.x][curr.y] {
	case '.':
		new_pos := Position{curr.x + direction.y, curr.y + direction.x}
		walk(arr, new_pos, direction, seen)
	case '|':
		if curr_direction == UP || curr_direction == DOWN {
			new_pos := Position{curr.x + direction.y, curr.y + direction.x}
			walk(arr, new_pos, direction, seen)
		} else {
			if (*seen)[curr] > 1 {
				return
			}
			walk(arr, curr, Position{0, -1}, seen)
			walk(arr, curr, Position{0, 1}, seen)
		}
	case '-':
		if curr_direction == LEFT || curr_direction == RIGHT {
			new_pos := Position{curr.x + direction.y, curr.y + direction.x}
			walk(arr, new_pos, direction, seen)
		} else {
			if (*seen)[curr] > 1 {
				return
			}
			walk(arr, curr, Position{-1, 0}, seen)
			walk(arr, curr, Position{1, 0}, seen)
		}
	case '/':
		if curr_direction == LEFT {
			direction = Position{0, 1}
		} else if curr_direction == RIGHT {
			direction = Position{0, -1}
		} else if curr_direction == UP {
			direction = Position{1, 0}
		} else if curr_direction == DOWN {
			direction = Position{-1, 0}
		}
		new_pos := Position{curr.x + direction.y, curr.y + direction.x}
		walk(arr, new_pos, direction, seen)
	case '\\':
		if curr_direction == LEFT {
			direction = Position{0, -1}
		} else if curr_direction == RIGHT {
			direction = Position{0, 1}
		} else if curr_direction == UP {
			direction = Position{-1, 0}
		} else if curr_direction == DOWN {
			direction = Position{1, 0}
		}
		new_pos := Position{curr.x + direction.y, curr.y + direction.x}
		walk(arr, new_pos, direction, seen)
	}
}

func get_direction(direction Position) int {
	if direction.x == 1 && direction.y == 0 {
		return RIGHT
	} else if direction.x == -1 && direction.y == 0 {
		return LEFT
	} else if direction.x == 0 && direction.y == -1 {
		return UP
	} else if direction.x == 0 && direction.y == 1 {
		return DOWN
	}
	return -1
}
