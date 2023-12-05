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
	card_instances := make(map[int]int)

	var winning_numbers [][]string
	var my_numbers [][]string
	index := 0

	for scanner.Scan() {
		card_instances[index] = 1

		temp_words := strings.Split(scanner.Text(), ":")
		numbers_string := strings.Split(temp_words[1], "|")

		winning_numbers = append(winning_numbers, strings.Fields(numbers_string[0]))
		my_numbers = append(my_numbers, strings.Fields(numbers_string[1]))
		index++
	}

	for index := range winning_numbers {
		card_wins := 0
		win_number_map := make(map[string]bool)

		for _, val := range winning_numbers[index] {
			win_number_map[val] = true
		}

		for _, val := range my_numbers[index] {
			_, exists := win_number_map[val]
			if exists {
				card_wins++
			}
		}

		for i := 0; i < card_wins; i++ {
			card_instances[index+1+i] += card_instances[index]
		}
		total += card_instances[index]
	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	fmt.Println("Result:", total)

	fmt.Println("Took", time.Since(start))
}
