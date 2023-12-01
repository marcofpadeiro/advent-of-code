package main

import (
	"bufio"
	"fmt"
	"os"
	"time"
	"unicode"
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
		firstInt := 0
		lastInt := 0

		line := []rune(scanner.Text())

		for i := 0; i < len(line); i++ {
			if unicode.IsDigit(line[i]) {
				firstInt = int(line[i]) - '0'
				break
			}
		}
		for i := len(line) - 1; i >= 0; i-- {
			if unicode.IsDigit(line[i]) {
				lastInt = int(line[i]) - '0'
				break
			}
		}

		total += int(firstInt)*10 + int(lastInt)
	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	fmt.Println("Result:", total)

	fmt.Println("Took", time.Since(start))
}
