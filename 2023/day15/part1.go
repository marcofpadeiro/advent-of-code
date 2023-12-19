package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

func main() {
	file, _ := os.Open("input.txt")

	defer file.Close()

	scanner := bufio.NewScanner(file)

	input := get_input(scanner)

	start := time.Now()

	total := 0

	for _, block := range input {
		sum := 0
		for _, c := range block {
			sum += int(c)
			sum *= 17
			sum %= 256
		}
		total += sum
	}

	time_took := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", time_took)
}

func get_input(scanner *bufio.Scanner) [][]rune {
	var temp [][]rune
	scanner.Scan()
	temp_arr := strings.Split(scanner.Text(), ",")
	for _, block := range temp_arr {
		temp = append(temp, []rune(block))
	}
	return temp
}
