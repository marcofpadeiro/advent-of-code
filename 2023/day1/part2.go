package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
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

	myMap := make(map[string]int)
	myMap["one"] = 1
	myMap["two"] = 2
	myMap["three"] = 3
	myMap["four"] = 4
	myMap["five"] = 5
	myMap["six"] = 6
	myMap["seven"] = 7
	myMap["eight"] = 8
	myMap["nine"] = 9

	for scanner.Scan() {
		firstInt := 0
		lastInt := 0

		line := []rune(scanner.Text())
		var tmpString string

	firstIntLoop:
		for i := 0; i < len(line); i++ {
			if unicode.IsDigit(line[i]) {
				firstInt = int(line[i]) - '0'
				break
			}

			for j := i; j < len(line); j++ {
				tmpString += string(line[j])

				var validEntries []string

				for key := range myMap {
					if strings.Contains(key, tmpString) {
						validEntries = append(validEntries, key)
					}
				}

				if len(validEntries) > 0 {
					for _, key := range validEntries {
						if strings.EqualFold(key, tmpString) {
							firstInt = myMap[key]
							break
						}
					}
				} else {
					tmpString = string(line[i])
					break
				}

				if firstInt != 0 {
					break firstIntLoop
				}
			}
		}

	lastIntLoop:
		for i := len(line) - 1; i >= 0; i-- {
			if unicode.IsDigit(line[i]) {
				lastInt = int(line[i]) - '0'
				break
			}

			for j := i; j >= 0; j-- {
				tmpString = string(line[j]) + tmpString

				var validEntries []string

				for key := range myMap {
					if strings.Contains(key, tmpString) {
						validEntries = append(validEntries, key)
					}
				}

				if len(validEntries) > 0 {
					for _, key := range validEntries {
						if strings.EqualFold(key, tmpString) {
							lastInt = myMap[key]
							break
						}
					}
				} else {
					tmpString = string(line[i])
					break
				}

				if lastInt != 0 {
					break lastIntLoop
				}
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
