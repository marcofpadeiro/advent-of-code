package main

import (
	"bufio"
	"fmt"
	"os"
	"reflect"
	"time"
)

const CYCLE_NUMBER = 1000000000

func main() {
	file, _ := os.Open("input.txt")

	defer file.Close()

	scanner := bufio.NewScanner(file)

	total := 0

	input := parse_input(scanner)

	start := time.Now()

	var seen [][][]rune
	seen = append(seen, create_copy(input))

	for {
		cycle(&input)

		idx := -1
		for i, x := range seen {
			if reflect.DeepEqual(x, input) {
				idx = i
				break
			}
		}
		if idx != -1 {
			cycle_len := len(seen) - idx
			final_idx := idx + (CYCLE_NUMBER-idx)%cycle_len
			total = weight(seen[final_idx])
			break
		}
		seen = append(seen, create_copy(input))
	}

	stop := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", stop)
}

func weight(input [][]rune) int {
	var total int
	for i := 0; i < len(input); i++ {
		counter := 0
		for j := 0; j < len(input[0]); j++ {
			if input[i][j] == 'O' {
				counter++
			}
		}
		total += counter * (len(input) - i)
	}
	return total
}

func parse_input(scanner *bufio.Scanner) [][]rune {
	var temp [][]rune
	for scanner.Scan() {
		temp = append(temp, []rune(scanner.Text()))
	}
	return temp
}

func roll_north(input *[][]rune) {
	for i := 0; i < len(*input); i++ {
		for j := 0; j < len((*input)[0]); j++ {
			if (*input)[i][j] == 'O' {
				limit := true
				index := i
				for limit {
					if index-1 == -1 || (*input)[index-1][j] != '.' {
						limit = false
						break
					}
					(*input)[index-1][j] = 'O'
					(*input)[index][j] = '.'
					index--
				}
			}
		}
	}
}

func cycle(input *[][]rune) {
	for i := 0; i < 4; i++ {
		roll_north(input)
		rotate(input)
	}
}

func rotate(input *[][]rune) {
	var temp [][]rune

	for x := 0; x < len((*input)[0]); x++ {
		var newRow []rune
		for y := 0; y < len(*input); y++ { // Fix: Increment y instead of x
			newRow = append(newRow, (*input)[len(*input)-1-y][x])
		}
		temp = append(temp, newRow)
	}
	(*input) = temp
}

func create_copy(input [][]rune) [][]rune {
	var temp [][]rune
	for _, line := range input {
		copiedLine := make([]rune, len(line))
		copy(copiedLine, line)

		temp = append(temp, copiedLine)
	}
	return temp
}
