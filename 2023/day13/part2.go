package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"time"
)

func main() {

	file, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	start := time.Now()
	total := 0

	patterns := parse_input(scanner)

	for _, pattern := range patterns {
		horizontal := search(transpose_matrice(pattern))
		var vertical int
		if horizontal == 0 {
			vertical = search(pattern)
		}
		total += vertical + horizontal*100
	}

	stop := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", stop)
}

func parse_input(scanner *bufio.Scanner) [][][]rune {
	var temp [][][]rune

	index := 0
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			index++
			continue
		}
		if index >= len(temp) {
			temp = append(temp, [][]rune{})
		}
		temp[index] = append(temp[index], []rune(line))

	}

	return temp
}

func search(pattern [][]rune) int {
	reflection := 0
	for x := 0; x < len(pattern[0])-1; x++ {
		smudges := 1
		for y := 0; y < len(pattern); y++ {
			if pattern[y][x] != pattern[y][x+1] {
				smudges--
				if smudges < 0 {
					break
				}
			}
		}
		if smudges >= 0 {
			if reflection == 0 {
				reflection = confirm_validity(pattern, x, smudges)
			}
		}
	}
	return reflection
}

func confirm_validity(pattern [][]rune, x, smudges int) int {
	minDistance := int(math.Min(float64(len(pattern[0])-2-x), float64(x)))

	for i := 1; i <= minDistance; i++ {
		for y := 0; y < len(pattern); y++ {
			if pattern[y][x-i] != pattern[y][x+1+i] {
				smudges--
				if smudges < 0 {
					break
				}
			}
		}
	}
	if smudges >= 0 {
		return x + 1
	}
	return 0
}

func transpose_matrice(pattern [][]rune) [][]rune {
	rows := len(pattern)
	cols := len(pattern[0])

	transposed := make([][]rune, cols)
	for i := range transposed {
		transposed[i] = make([]rune, rows)
		for j := range transposed[i] {
			transposed[i][j] = pattern[j][i]
		}
	}

	return transposed

}
