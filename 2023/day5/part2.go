package main

import (
	"bufio"
	"fmt"
	"math"
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

func getSeeds(line string) []int {
	seeds_str := strings.Split(line, ":")
	seeds_arr := strings.Fields(seeds_str[1])

	var seeds []int
	for _, seed := range seeds_arr {
		seed_int, _ := strconv.Atoi(seed)
		seeds = append(seeds, seed_int)
	}
	return seeds
}

func getBlocks(lines []string) [7][]Instruction {
	var blocks [7][]Instruction
	currBlock := 0
	for i := 0; i < len(lines); i++ {
		if lines[i] == "" {
			i++
			currBlock++
			continue
		}

		numbers_str := strings.Fields(lines[i])
		var numbers [3]int
		for n_index, n := range numbers_str {
			temp_int, _ := strconv.Atoi(n)
			numbers[n_index] = temp_int
		}

		blocks[currBlock] = append(blocks[currBlock], Instruction{numbers[1], numbers[1] + numbers[2], numbers[0]})
	}
	return blocks
}

func main() {
	file, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	start := time.Now()

	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	seeds := getSeeds(lines[0])

	blocks := getBlocks(lines[3:])

	lowest := math.MaxInt64

	for i := 0; i < len(seeds); i += 2 {
		for j := seeds[i]; j < seeds[i]+seeds[i+1]; j++ {
			seed := j
			for _, block := range blocks {
				for _, instruction := range block {
					if seed >= instruction.source_start && seed < instruction.source_end {
						seed = seed - instruction.source_start + instruction.dest_start
						break
					}
				}
			}
			if seed < lowest {
				lowest = seed
			}
		}
	}

	time := time.Since(start)

	fmt.Println("Result:", lowest)

	fmt.Println("Took", time)

}
