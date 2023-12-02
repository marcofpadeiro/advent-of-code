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

		colors['r'] = 0
		colors['g'] = 0
		colors['b'] = 0

		for _, set := range sets {
			cubes := strings.Split(set, ",")

			for _, cube := range cubes {
				var number int
				var color string
				_, err := fmt.Sscanf(cube, " %d %s", &number, &color)
				if err != nil {
					fmt.Println(err)
				}

				if number > colors[rune(color[0])] {
					colors[rune(color[0])] = number
				}
			}
		}
		total += colors['r'] * colors['g'] * colors['b']
	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	fmt.Println("Result:", total)

	fmt.Println("Took", time.Since(start))
}
