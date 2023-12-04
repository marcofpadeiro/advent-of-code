package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
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

	var lines [][]rune

	for scanner.Scan() {
		lines = append(lines, []rune(scanner.Text()))
	}

	for line_index, line := range lines {
		inNumber := false
		var leftPointer int
		var temp string
		for i := 0; i < len(line); i++ {
			if unicode.IsDigit(line[i]) {
				if !inNumber {
					leftPointer = i
					inNumber = true
					temp = ""
				}
				temp += string(line[i])

				if i == len(line)-1 {
					number, _ := strconv.Atoi(temp)
					inNumber = false
					rightPointer := i - 1

					if checkIfNumberLegal(lines, line_index, i, leftPointer, rightPointer) {
						total += number
					}
				}

			} else {
				if inNumber {
					number, _ := strconv.Atoi(temp)
					inNumber = false
					rightPointer := i - 1

					if checkIfNumberLegal(lines, line_index, i, leftPointer, rightPointer) {
						total += number
					}

				}
			}
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	fmt.Println("Result:", total)

	fmt.Println("Took", time.Since(start))
}

// unicode.IsSymbol was not enough because it didn't register all needed symbols
func isSymbol(char rune) bool {
	if !unicode.IsDigit(char) && char != 46 {
		return true
	}
	return false

}

func checkIfNumberLegal(lines [][]rune, line_index, col_index, leftPointer, rightPointer int) bool {
	isValid := false

	topMargin, rightMargin, bottomMargin, leftMargin := line_index, rightPointer, line_index, leftPointer

	if line_index > 0 {
		topMargin = line_index - 1
	}
	if rightPointer < len(lines[line_index])-1 {
		rightMargin = rightPointer + 1
	}
	if line_index < len(lines)-1 {
		bottomMargin = line_index + 1
	}
	if leftPointer > 0 {
		leftMargin = leftPointer - 1
	}

	for i := leftMargin; i <= rightMargin; i++ {
		if isSymbol(lines[topMargin][i]) || isSymbol(lines[bottomMargin][i]) {
			isValid = true
		}
	}

	if isSymbol(lines[line_index][leftMargin]) || isSymbol(lines[line_index][rightMargin]) {
		isValid = true
	}

	return isValid
}

