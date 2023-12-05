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
	for scanner.Scan() {
		card_total := 0
		correct_numbers := 0
		win_number_map := make(map[string]bool)
		temp_words := strings.Split(scanner.Text(), ":")
		numbers_string := strings.Split(temp_words[1], "|")

		winning_numbers := strings.Fields(numbers_string[0])
		my_numbers := strings.Fields(numbers_string[1])

		for _, val := range winning_numbers {
			win_number_map[val] = true
		}

		for _, val := range my_numbers {
			_, exists := win_number_map[val]
			if exists {
				if correct_numbers == 0 {
					card_total++
				} else {
					card_total *= 2
				}
				correct_numbers++
			}
		}
		total += card_total
	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	fmt.Println("Result:", total)

	fmt.Println("Took", time.Since(start))
}
