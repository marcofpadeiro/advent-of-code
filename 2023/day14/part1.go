package main

import (
	"bufio"
	"fmt"
	"os"
	"time"
)

func main() {
	file, _ := os.Open("input.txt")

	defer file.Close()

	scanner := bufio.NewScanner(file)

	total := 0

	input := parse_input(scanner)

	start := time.Now()

	roll(&input)

	for i := 0; i < len(input); i++ {
		counter := 0
		for j := 0; j < len(input[0]); j++ {
			if input[i][j] == 'O' {
				counter++
			}
		}
		total += counter * (len(input) - i)

	}
	stop := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", stop)
}

func parse_input(scanner *bufio.Scanner) [][]rune {
	var temp [][]rune
	for scanner.Scan() {
		temp = append(temp, []rune(scanner.Text()))
	}
	return temp
}

func roll(input *[][]rune) {
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
