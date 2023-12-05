package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type Instruction struct {
	source_start int
	source_end   int
	dest_start   int
}

func main() {
	file, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	start := time.Now()

	var seeds []int
	var lines []string
	var instructions []Instruction

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	seeds_str := strings.Split(lines[0], ":")
	seeds_arr := strings.Fields(seeds_str[1])

	for _, seed := range seeds_arr {
		seed_int, _ := strconv.Atoi(seed)
		seeds = append(seeds, seed_int)
	}

	for i := 3; i < len(lines); i++ {
		if lines[i] == "" || i == len(lines)-1 {
			if len(instructions) > 0 {
				for index, seed := range seeds {
					for _, instruction := range instructions {
						if seed >= instruction.source_start && seed < instruction.source_end {
							seeds[index] = seed - instruction.source_start + instruction.dest_start
						}
					}
				}
			}

			i++
			instructions = nil
			continue
		}

		numbers_str := strings.Fields(lines[i])
		var numbers [3]int
		for n_index, n := range numbers_str {
			temp_int, _ := strconv.Atoi(n)
			numbers[n_index] = temp_int
		}

		instructions = append(instructions, Instruction{numbers[1], numbers[1] + numbers[2], numbers[0]})
	}

	// insert seed value and go through maps
	lowest := seeds[0]
	for i := 1; i < len(seeds); i++ {
		if seeds[i] < lowest {
			lowest = seeds[i]
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	time := time.Since(start)
	fmt.Println("Result:", lowest)

	fmt.Println("Took", time)
}
