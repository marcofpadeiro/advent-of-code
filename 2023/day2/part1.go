package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
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

	colors := make(map[rune]int)
	for scanner.Scan() {

		var game int
		temp_words := strings.Split(scanner.Text(), ":")
		fmt.Sscanf(temp_words[0], "Game %d:", &game)
		sets := strings.Split(temp_words[1], ";")

		isValid := true
		for _, set := range sets {
			colors['b'] = 0
			colors['g'] = 0
			colors['r'] = 0
			fmt.Println(set)
			cubes := strings.Split(set, ",")

			for _, cube := range cubes {
				var number int
				var color string
				_, err := fmt.Sscanf(cube, " %d %s", &number, &color)
				if err != nil {
					fmt.Println(err)
				}

				colors[rune(color[0])] += number
			}
			if colors['r'] > 12 || colors['g'] > 13 || colors['b'] > 14 {
				isValid = false
				break
			}
		}
		if isValid {
			total += game
		}

	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	fmt.Println("Result:", total)

	fmt.Println("Took", time.Since(start))
}
